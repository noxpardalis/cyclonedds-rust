use cyclonedds as dds;

mod common;

#[test]
fn read_write() -> dds::Result<()> {
    let domain_id_01 = common::domain::unique_id();
    let domain_id_02 = common::domain::unique_id();
    let external_domain_id = domain_id_02;
    let topic_name = common::topic::unique_name();

    let domain_01 = dds::Domain::new_with_xml_config(
        domain_id_01,
        &format!(
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
        ),
    )?;
    let domain_02 = dds::Domain::new_with_xml_config(
        domain_id_02,
        &format!(
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
        ),
    )?;
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
