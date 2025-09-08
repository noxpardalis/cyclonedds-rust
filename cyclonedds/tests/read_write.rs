use cyclonedds as dds;

mod common;

#[test]
fn read_write() -> dds::Result<()> {
    let domain_id = common::domain::unique_id();
    let topic_name = common::topic::unique_name();

    let domain = dds::Domain::new(domain_id)?;
    let qos = dds::QoS::new().with_history(dds::qos::policy::History::KeepLast { depth: 3 });
    let participant = dds::Participant::new_with_qos(&domain, &qos)?;
    let topic = dds::Topic::new_with_qos(&participant, &topic_name, &qos)?;
    let publisher = dds::Publisher::new_with_qos(&participant, &qos)?;
    let writer = dds::Writer::new_with_qos(&publisher, &topic, &qos)?;
    let subscriber = dds::Subscriber::new_with_qos(&participant, &qos)?;
    let reader = dds::Reader::new_with_qos(&subscriber, &topic, &qos)?;

    let sample = common::topic::Data::default();
    writer.write(&sample)?;
    writer.write(&sample)?;
    let samples = reader
        .take()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(samples.len(), 2);
    assert_eq!(*samples[0], sample);
    assert_eq!(*samples[1], sample);

    let sample_01 = common::topic::Data::default();
    let sample_02 = common::topic::Data {
        x: 100,
        y: 200,
        message: format!("This is a sample: 🐋鯨❤️"),
    };
    writer.write(&sample_01)?;
    writer.write(&sample_02)?;
    let samples = reader
        .read()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(samples.len(), 2);
    assert_eq!(*samples[0], sample_01);
    assert_eq!(*samples[1], sample_02);

    let sample_03 = common::topic::Data {
        x: 101,
        y: 202,
        message: format!("🐋鯨❤️"),
    };
    writer.write(&sample_03)?;
    let samples = reader
        .take()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(samples.len(), 3);
    assert_eq!(*samples[0], sample_01);
    assert_eq!(*samples[1], sample_02);
    assert_eq!(*samples[2], sample_03);

    let samples = reader.read()?;
    assert_eq!(samples.len(), 0);

    Ok(())
}
