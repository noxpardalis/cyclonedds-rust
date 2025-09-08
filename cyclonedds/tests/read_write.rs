use cyclonedds as dds;

mod common;

#[test]
fn read_write_generate_keyhash() -> dds::Result<()> {
    let domain_id_01 = common::domain::unique_id();
    let domain_id_02 = common::domain::unique_id();
    let external_domain_id = domain_id_02;
    let topic_name = common::topic::unique_name();

    let config = &format!(
        "<Discovery>
           <ExternalDomainId>
             {external_domain_id}
           </ExternalDomainId>
         </Discovery>
         <Internal>
           <GenerateKeyhash>
             true
           </GenerateKeyhash>
         </Internal>"
    );

    let qos = dds::QoS::new()
        .with_history(dds::qos::policy::History::KeepAll)
        .with_durability(dds::qos::policy::Durability::TransientLocal);

    let domain = dds::Domain::new_with_xml_config(domain_id_01, config)?;
    let participant = dds::Participant::builder(&domain).with_qos(&qos).build()?;
    let topic = dds::Topic::<common::topic::Data>::new(&participant, &topic_name)?;
    let writer = dds::Writer::new(&topic)?;

    let domain = dds::Domain::new_with_xml_config(domain_id_02, config)?;
    let participant = dds::Participant::builder(&domain).with_qos(&qos).build()?;
    let topic = dds::Topic::<common::topic::Data>::new(&participant, &topic_name)?;
    let reader = dds::Reader::new(&topic)?;

    std::thread::sleep(std::time::Duration::from_millis(100));

    let sample = common::topic::Data {
        x: 12,
        y: 35,
        message: format!("message from {topic_name}"),
    };
    writer.write(&sample)?;

    while reader.peek()?.len() < 1 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let samples = reader.read()?;
    assert_eq!(samples.len(), 1);
    assert_eq!(*samples[0], sample);
    // todo!("this hits the get_keyhash but not the from_keyhash");
    Ok(())
}

#[test]
fn read_write_same_type_shape_different_sertype() -> dds::Result<()> {
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
    struct Data01 {
        x: u32,
        y: u32,
        z: u32,
    }

    impl dds::Topicable for Data01 {
        type Key = ();

        fn from_key(_: &Self::Key) -> Self {
            Default::default()
        }

        fn into_key(&self) -> Self::Key {
            ()
        }

        fn type_name() -> impl AsRef<str> {
            "Data"
        }
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
    struct Data02 {
        x: u32,
        y: u32,
        z: u32,
    }

    impl dds::Topicable for Data02 {
        type Key = ();

        fn from_key(_: &Self::Key) -> Self {
            Default::default()
        }

        fn into_key(&self) -> Self::Key {
            ()
        }

        fn type_name() -> impl AsRef<str> {
            "Data"
        }
    }

    impl std::cmp::PartialEq<Data02> for Data01 {
        fn eq(&self, other: &Data02) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    let domain_id = common::domain::unique_id();
    let topic_name = common::topic::unique_name();
    let domain = dds::Domain::new(domain_id)?;
    let participant = dds::Participant::new(&domain)?;
    let topic01 = dds::Topic::<Data01>::new(&participant, &topic_name)?;
    let topic02 = dds::Topic::<Data02>::new(&participant, &topic_name)?;

    let writer = dds::Writer::new(&topic01)?;
    let reader = dds::Reader::new(&topic02)?;

    let sample = Data01 {
        x: 1234,
        y: 5678,
        z: 9101112,
    };

    writer.write(&sample)?;

    let samples = reader.take()?;

    assert_eq!(samples.len(), 1);
    assert_eq!(sample, *samples[0]);

    Ok(())
}

#[test]
fn read_write() -> dds::Result<()> {
    let domain_id_01 = common::domain::unique_id();
    let domain_id_02 = common::domain::unique_id();
    let external_domain_id = domain_id_02;
    let topic_name = common::topic::unique_name();
    let config = &format!(
        "<Domain>
          <Tracing>
            <OutputFile>
              stdout
            </OutputFile>
            <Verbosity>
              finest
            </Verbosity>
          </Tracing>
          <Discovery>
            <ExternalDomainId>
              {external_domain_id}
            </ExternalDomainId>
          </Discovery>
        </Domain>"
    );

    let domain_01 = dds::Domain::new_with_xml_config(domain_id_01, config)?;
    let domain_02 = dds::Domain::new_with_xml_config(domain_id_02, config)?;
    let qos = dds::QoS::new()
        .with_history(dds::qos::policy::History::KeepAll)
        .with_durability(dds::qos::policy::Durability::TransientLocal);

    let participant_01 = dds::Participant::builder(&domain_01)
        .with_qos(&qos)
        .build()?;
    let topic_01: dds::Topic<common::topic::Data> =
        dds::Topic::builder(&participant_01, &topic_name)
            .with_qos(&qos)
            .build()?;
    let subscriber = dds::Subscriber::builder(&participant_01)
        .with_qos(&qos)
        .build()?;
    let reader = dds::Reader::builder(&topic_01)
        .with_participant_or_subscriber(&subscriber)
        .with_qos(&qos)
        .build()?;

    let participant_02 = dds::Participant::builder(&domain_02)
        .with_qos(&qos)
        .build()?;
    let topic_02 = dds::Topic::builder(&participant_02, &topic_name)
        .with_qos(&qos)
        .build()?;
    let publisher = dds::Publisher::builder(&participant_02)
        .with_qos(&qos)
        .build()?;
    let writer = dds::Writer::builder(&topic_02)
        .with_participant_or_publisher(&publisher)
        .with_qos(&qos)
        .build()?;

    std::thread::sleep(std::time::Duration::from_millis(10));

    let sample = common::topic::Data::default();
    writer.write(&sample)?;

    while reader.peek()?.len() < 1 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let samples = reader.take()?;

    assert_eq!(samples.len(), 1);
    assert_eq!(*samples[0], sample);

    let samples = reader.take()?;
    assert!(samples.is_empty());

    let sample_01 = common::topic::Data::default();
    let sample_02 = common::topic::Data {
        x: 100,
        y: 200,
        message: format!("This is a sample: 🐋鯨❤️"),
    };

    writer.write(&sample_01)?;
    writer.write(&sample_02)?;

    while reader.peek()?.len() < 2 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let samples = reader.read()?;

    assert_eq!(samples.len(), 2);
    assert_eq!(*samples[0], sample_01);
    assert_eq!(*samples[1], sample_02);

    let sample_03 = common::topic::Data {
        x: 101,
        y: 202,
        message: format!("🐋鯨❤️"),
    };
    writer.write(&sample_03)?;

    while reader.peek()?.len() < 3 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let samples = reader.take()?;

    assert_eq!(samples.len(), 3);
    assert_eq!(*samples[0], sample_01);
    assert_eq!(*samples[1], sample_02);
    assert_eq!(*samples[2], sample_03);

    let samples = reader.read()?;
    assert_eq!(samples.len(), 0);

    Ok(())
}
