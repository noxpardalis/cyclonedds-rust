use crate::Result;
use crate::entity::Entity;
use crate::internal::ffi;

///
#[derive(Debug)]
pub struct GuardCondition<'owner> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'owner dyn Entity>,
}

impl<'o> GuardCondition<'o> {
    ///
    pub fn new(owner: &'o dyn Entity) -> Result<Self> {
        let owner = owner.id().inner;
        let inner = ffi::dds_create_guardcondition(owner)?;
        Ok(Self {
            inner,
            phantom: std::marker::PhantomData,
        })
    }

    ///
    pub fn set(&mut self, triggered: bool) -> Result<()> {
        ffi::dds_set_guardcondition(self.inner, triggered)
    }

    ///
    pub fn peek(&self) -> Result<bool> {
        self.read()
    }

    ///
    pub fn read(&self) -> Result<bool> {
        ffi::dds_read_guardcondition(self.inner)
    }

    ///
    pub fn take(&mut self) -> Result<bool> {
        ffi::dds_take_guardcondition(self.inner)
    }
}

impl Drop for GuardCondition<'_> {
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_condition_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let _ = GuardCondition::new(&participant).unwrap();
    }

    #[test]
    fn test_guard_condition_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let mut participant = crate::Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = GuardCondition::new(&participant).unwrap_err();
        participant.inner = participant_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_guard_condition_set() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut guard_condition = GuardCondition::new(&participant).unwrap();

        guard_condition.set(true).unwrap();
        guard_condition.set(false).unwrap();
    }

    #[test]
    fn test_guard_condition_peek() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut guard_condition = GuardCondition::new(&participant).unwrap();

        guard_condition.set(true).unwrap();
        let triggered = guard_condition.peek().unwrap();
        assert_eq!(triggered, true);
        let triggered = guard_condition.peek().unwrap();
        assert_eq!(triggered, true);

        guard_condition.set(false).unwrap();
        let triggered = guard_condition.peek().unwrap();
        assert_eq!(triggered, false);
        let triggered = guard_condition.peek().unwrap();
        assert_eq!(triggered, false);
    }

    #[test]
    fn test_guard_condition_read() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut guard_condition = GuardCondition::new(&participant).unwrap();

        guard_condition.set(true).unwrap();
        let triggered = guard_condition.read().unwrap();
        assert_eq!(triggered, true);
        let triggered = guard_condition.read().unwrap();
        assert_eq!(triggered, true);

        guard_condition.set(false).unwrap();
        let triggered = guard_condition.read().unwrap();
        assert_eq!(triggered, false);
        let triggered = guard_condition.read().unwrap();
        assert_eq!(triggered, false);
    }

    #[test]
    fn test_guard_condition_take() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut guard_condition = GuardCondition::new(&participant).unwrap();

        guard_condition.set(true).unwrap();
        let triggered = guard_condition.take().unwrap();
        assert_eq!(triggered, true);
        let triggered = guard_condition.take().unwrap();
        assert_eq!(triggered, false);

        guard_condition.set(false).unwrap();
        let triggered = guard_condition.take().unwrap();
        assert_eq!(triggered, false);
        let triggered = guard_condition.take().unwrap();
        assert_eq!(triggered, false);
    }

    #[test]
    fn test_guard_condition_operations_on_invalid_guard_condition() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut guard_condition = GuardCondition::new(&participant).unwrap();
        let guard_condition_id = guard_condition.inner;
        guard_condition.inner = 0;

        let result = guard_condition.set(false).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = guard_condition.read().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = guard_condition.take().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);

        guard_condition.inner = guard_condition_id;
    }
}
