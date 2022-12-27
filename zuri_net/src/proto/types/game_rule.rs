use crate::proto::io::{Reader, Writer};

#[derive(Debug)]
pub struct GameRule {
    pub name: String,
    pub can_be_modified_by_player: bool,
    //pub value: dyn Any, // TODO
}

impl GameRule {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.can_be_modified_by_player);
        //writer.write_TODO(self.value);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            can_be_modified_by_player: reader.bool(),
            //value: reader.read_TODO(),
        }
    }
}
