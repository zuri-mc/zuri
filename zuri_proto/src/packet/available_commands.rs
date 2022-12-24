use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct AvailableCommands {
    pub commands: Vec<Command>,
    pub constraints: Vec<CommandEnumConstraint>,
}

impl AvailableCommands {
    fn enum_values(&self) -> (Vec<String>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for alias in &command.aliases {
                if !indices.contains_key(alias.as_str()) {
                    indices.insert(alias.clone(), values.len());
                    values.push(alias.clone());
                }
            }
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    for option in &parameter.command_enum.options {
                        if !indices.contains_key(option) {
                            indices.insert(option.clone(), values.len());
                            values.push(option.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }

    fn suffixes(&self) -> (Vec<String>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if !parameter.suffix.is_empty() {
                        if !indices.contains_key(&parameter.suffix) {
                            indices.insert(parameter.suffix.clone(), values.len());
                            values.push(parameter.suffix.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }

    fn enums(&self) -> (Vec<CommandEnum>, BTreeMap<String, usize>) {
        todo!()
        // let mut values = Vec::new();
        // let mut indices = BTreeMap::new();
        // for command in self.commands {
        //     if !command.aliases.is_empty() {
        //         let alias_enum = CommandEnum {
        //             enum_type: format!("{}Aliases", command.name),
        //             options: command.aliases,
        //             ..Default::default()
        //         };
        //         indices.insert(alias_enum.enum_type, values.len());
        //         values.push(alias_enum);
        //     }
        //     for overload in command.overloads {
        //         for parameter in overload.parameters {
        //             if !parameter.command_enum.options.is_empty() && !parameter.command_enum.dynamic {
        //                 if !indices.contains_key(&parameter.command_enum.enum_type) {
        //                     indices.insert(parameter.command_enum.enum_type.clone(), values.len());
        //                     values.push(parameter.command_enum);
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // (values, indices)
    }

    fn dynamic_enums(&self) -> (Vec<CommandEnum>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if parameter.command_enum.dynamic {
                        if !indices.contains_key(&parameter.command_enum.enum_type) {
                            indices.insert(parameter.command_enum.enum_type.clone(), values.len());
                            values.push(parameter.command_enum.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }
}

impl Packet for AvailableCommands {
    fn write(&self, writer: &mut Writer) {
        todo!()
        // (values, valueIndices) = self.enum_values();
        // (suffixes, suffixIndices) = self.suffixes();
        // (enums, enumIndices) = self.enums();
        // (dynamicEnums, dynamicEnumIndices) = self.dynamic_enums();
        //
        // writer.var_u32(values.len() as u32);
        // values.iter().for_each(|value| writer.string(value));
        // writer.var_u32(suffixes.len() as u32);
        // suffixes.iter().for_each(|suffix| writer.string(suffix));
        //
        // writer.var_u32(enums.len() as u32);
        // enums.iter().for_each(|command_enum| command_enum.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        todo!()
        // Self {
        //     LEN: reader.read_TODO(),
        //     commands: reader.read_Command(),
        //     LEN: reader.read_TODO(),
        //     constraints: reader.read_CommandEnumConstraint(),
        // }
    }
}
