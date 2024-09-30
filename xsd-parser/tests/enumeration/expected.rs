#[derive(PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub enum FooType {
    #[yaserde(prefix = "tns", rename = "OFF")]
    Off,
    #[yaserde(prefix = "tns", rename = "ON")]
    On,
    #[yaserde(prefix = "tns", rename = "AUTO")]
    Auto,
    __Unknown__(String),
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}
impl Validate for FooType {}


#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType2(pub String);

impl Validate for FooType2 {}

