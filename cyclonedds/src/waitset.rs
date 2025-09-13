use crate::Participant;
use crate::Result;
use crate::entity::Entity;
use crate::entity::EntityId;
use crate::internal::ffi;

///
pub struct WaitSet<'domain, 'participant, 'attached> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    attached: std::collections::HashMap<EntityId, &'attached dyn Entity>,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

impl std::fmt::Debug for WaitSet<'_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WaitSet")
            .field("inner", &self.inner)
            .field("attached", &self.attached.keys())
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<'d, 'p, 'a> WaitSet<'d, 'p, 'a> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        let inner = ffi::dds_create_waitset(participant.inner)?;
        Ok(Self {
            inner,
            attached: Default::default(),
            phantom: Default::default(),
        })
    }

    ///
    pub fn attach<T>(&mut self, entity: &'a dyn Entity, blob: Option<&'a T>) -> Result<()> {
        // TODO: use dyn Any instead for the attachment to allow for a heterogenous attachment?
        let id = entity.id();
        if !self.attached.contains_key(&id) {
            ffi::dds_waitset_attach(
                self.inner,
                id.inner,
                blob.map(|blob| blob as *const _)
                    .unwrap_or(std::ptr::null()) as isize,
            )?;
            self.attached.insert(id, entity);
        }
        Ok(())
    }

    ///
    pub fn detach(&mut self, entity: &'a dyn Entity) -> Result<()> {
        let entity = entity.id();
        self.detach_id(entity)
    }

    ///
    fn detach_id(&mut self, entity_id: EntityId) -> Result<()> {
        if self.attached.contains_key(&entity_id) {
            ffi::dds_waitset_detach(self.inner, entity_id.inner)?;
            self.attached.remove(&entity_id);
        }

        Ok(())
    }

    ///
    pub fn set_trigger(&mut self, trigger: bool) -> Result<()> {
        ffi::dds_waitset_set_trigger(self.inner, trigger)
    }

    ///
    pub fn wait(&mut self, timeout: crate::Duration) -> Result<i32> {
        ffi::dds_waitset_wait(self.inner, timeout.inner)
    }

    ///
    pub fn wait_until(&mut self, absolute_time: crate::Time) -> Result<i32> {
        ffi::dds_waitset_wait_until(self.inner, absolute_time.inner)
    }

    ///
    pub fn is_attached(&self, entity: &'a dyn Entity) -> bool {
        self.attached.contains_key(&entity.id())
    }
}

impl Drop for WaitSet<'_, '_, '_> {
    fn drop(&mut self) {
        for (entity_id, _) in &self.attached {
            let result = ffi::dds_waitset_detach(self.inner, entity_id.inner);
            debug_assert!(dbg!(result).is_ok());
        }

        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;

    #[test]
    fn test_waitset_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let _ = WaitSet::new(&participant).unwrap();
    }

    #[test]
    fn test_waitset_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let mut participant = crate::Participant::new(&domain, None).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = WaitSet::new(&participant).unwrap_err();
        participant.inner = participant_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_waitset_debug_formatting() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let waitset = WaitSet::new(&participant).unwrap();

        let result = format!("{waitset:?}");
        assert!(result.contains(&format!("{}", waitset.inner)));
    }

    #[test]
    fn test_waitset_attachment() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let topic = crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None)
            .unwrap();
        let reader = crate::Reader::new(&participant, &topic, None).unwrap();
        let mask = state::sample::Any | state::view::Any | state::instance::Any;
        let read_condition = crate::ReadCondition::new(&reader, mask).unwrap();

        let mut waitset = WaitSet::new(&participant).unwrap();

        let result = waitset.attach::<()>(&topic, None);
        assert!(result.is_ok());
        let result = waitset.attach::<()>(&topic, None);
        assert!(result.is_ok());
        let result = waitset.attach::<()>(&read_condition, None);
        assert!(result.is_ok());

        assert!(waitset.is_attached(&topic));
        assert!(waitset.is_attached(&read_condition));

        let result = waitset.detach(&read_condition);
        assert!(result.is_ok());

        assert!(waitset.is_attached(&topic));
        assert!(!waitset.is_attached(&read_condition));

        let result = waitset.detach(&read_condition);
        assert!(result.is_ok());
    }

    #[test]
    fn test_waitset_attachment_with_invalid_waitset() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let topic = crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None)
            .unwrap();
        let reader = crate::Reader::new(&participant, &topic, None).unwrap();
        let mask = state::sample::Any | state::view::Any | state::instance::Any;
        let read_condition = crate::ReadCondition::new(&reader, mask).unwrap();

        let mut waitset = WaitSet::new(&participant).unwrap();

        let result = waitset.attach::<()>(&topic, None);
        assert!(result.is_ok());

        let waitset_id = waitset.inner;
        waitset.inner = 0;

        let result = waitset.attach::<()>(&read_condition, None).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);

        let result = waitset.detach(&topic).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);

        waitset.inner = waitset_id;
    }

    #[test]
    fn test_waitset_wait() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let topic = crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None)
            .unwrap();
        let reader = crate::Reader::new(&participant, &topic, None).unwrap();
        let writer = crate::Writer::new(&participant, &topic, None).unwrap();
        let mask = state::sample::Any | state::view::Any | state::instance::Any;
        let read_condition_1 = crate::ReadCondition::new(&reader, mask).unwrap();

        let mask = state::sample::Any | state::view::Any | state::instance::Any;
        let read_condition_2 = crate::ReadCondition::new(&reader, mask).unwrap();

        let mut waitset = WaitSet::new(&participant).unwrap();
        waitset
            .attach::<i32>(&read_condition_1, Some(&10))
            .unwrap();

        let actual = waitset
            .wait(crate::Duration::from_nanos(5_000_000))
            .unwrap();
        assert_eq!(actual, 0);
        writer.write(&Default::default()).unwrap();
        let actual = waitset
            .wait(crate::Duration::from_nanos(1_000_000_000))
            .unwrap();
        assert_eq!(actual, 1);
        waitset
            .attach::<i32>(&read_condition_2, Some(&20))
            .unwrap();
        let actual = waitset
            .wait(crate::Duration::from_nanos(1_000_000_000))
            .unwrap();
        assert_eq!(actual, 2);

        let actual = waitset.wait_until(crate::Time::from_nanos(1)).unwrap();
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_waitset_set_trigger() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let mut waitset = WaitSet::new(&participant).unwrap();

        let result = waitset.set_trigger(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_waitset_set_trigger_with_invalid_waitset() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let mut waitset = WaitSet::new(&participant).unwrap();
        let waitset_id = waitset.inner;
        waitset.inner = 0;

        let result = waitset.set_trigger(true).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        waitset.inner = waitset_id;
    }
}
