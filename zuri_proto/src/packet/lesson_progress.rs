#[derive(Debug)]
pub struct LessonProgress {
    pub action: LessonAction,
    pub score: i32,
    pub identifier: String,
}

impl Packet for LessonProgress {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action).unwrap());
        writer.var_i32(self.score);
        writer.string(self.identifier.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            score: reader.var_i32(),
            identifier: reader.string(),
        }
    }
}
