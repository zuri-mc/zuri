use bytes::Bytes;

use crate::proto::io::{Reader, Writer};

#[derive(Debug, Default, Clone)]
pub struct Skin {
    pub skin_id: String,
    pub play_fab_id: String,
    pub skin_resource_patch: Bytes,
    pub skin_image_width: u32,
    pub skin_image_height: u32,
    pub skin_data: Bytes,
    pub animations: Vec<SkinAnimation>,
    pub cape_image_width: u32,
    pub cape_image_height: u32,
    pub cape_data: Bytes,
    pub skin_geometry: Bytes,
    pub geometry_data_engine_version: Bytes,
    pub animation_data: Bytes,
    pub cape_id: String,
    pub full_id: String,
    pub arm_size: String,
    pub skin_colour: String,
    pub persona_pieces: Vec<PersonaPiece>,
    pub piece_tint_colours: Vec<PersonaPieceTintColour>,
    pub premium_skin: bool,
    pub persona_skin: bool,
    pub persona_cape_on_classic_skin: bool,
    pub primary_user: bool,
    pub trusted: bool,
}

impl Skin {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.skin_id.as_str());
        writer.string(self.play_fab_id.as_str());
        writer.byte_slice(&self.skin_resource_patch);
        writer.u32(self.skin_image_width);
        writer.u32(self.skin_image_height);
        writer.byte_slice(&self.skin_data);
        writer.u32(self.animations.len() as u32);
        self.animations.iter().for_each(|animation| animation.write(writer));
        writer.u32(self.cape_image_width);
        writer.u32(self.cape_image_height);
        writer.byte_slice(&self.cape_data);
        writer.byte_slice(&self.skin_geometry);
        writer.byte_slice(&self.geometry_data_engine_version);
        writer.byte_slice(&self.animation_data);
        writer.string(self.cape_id.as_str());
        writer.string(self.full_id.as_str());
        writer.string(self.arm_size.as_str());
        writer.string(self.skin_colour.as_str());
        writer.u32(self.persona_pieces.len() as u32);
        self.persona_pieces.iter().for_each(|piece| piece.write(writer));
        writer.u32(self.piece_tint_colours.len() as u32);
        self.piece_tint_colours.iter().for_each(|colour| colour.write(writer));
        writer.bool(self.premium_skin);
        writer.bool(self.persona_skin);
        writer.bool(self.persona_cape_on_classic_skin);
        writer.bool(self.primary_user);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            skin_id: reader.string(),
            play_fab_id: reader.string(),
            skin_resource_patch: reader.byte_slice(),
            skin_image_width: reader.u32(),
            skin_image_height: reader.u32(),
            skin_data: reader.byte_slice(),
            animations: (0..reader.u32()).map(|_| SkinAnimation::read(reader)).collect(),
            cape_image_width: reader.u32(),
            cape_image_height: reader.u32(),
            cape_data: reader.byte_slice(),
            skin_geometry: reader.byte_slice(),
            geometry_data_engine_version: reader.byte_slice(),
            animation_data: reader.byte_slice(),
            cape_id: reader.string(),
            full_id: reader.string(),
            arm_size: reader.string(),
            skin_colour: reader.string(),
            persona_pieces: (0..reader.u32()).map(|_| PersonaPiece::read(reader)).collect(),
            piece_tint_colours: (0..reader.u32()).map(|_| PersonaPieceTintColour::read(reader)).collect(),
            premium_skin: reader.bool(),
            persona_skin: reader.bool(),
            persona_cape_on_classic_skin: reader.bool(),
            primary_user: reader.bool(),
            trusted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkinAnimation {
    pub image_width: u32,
    pub image_height: u32,
    pub image_data: Bytes,
    pub animation_type: u32,
    pub frame_count: f32,
    pub expression_type: u32,
}

impl SkinAnimation {
    pub fn write(&self, writer: &mut Writer) {
        writer.u32(self.image_width);
        writer.u32(self.image_height);
        writer.byte_slice(&self.image_data);
        writer.u32(self.animation_type);
        writer.f32(self.frame_count);
        writer.u32(self.expression_type);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            image_width: reader.u32(),
            image_height: reader.u32(),
            image_data: reader.byte_slice(),
            animation_type: reader.u32(),
            frame_count: reader.f32(),
            expression_type: reader.u32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PersonaPiece {
    pub piece_id: String,
    pub piece_type: String,
    pub pack_id: String,
    pub default: bool,
    pub product_id: String,
}

impl PersonaPiece {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.piece_id.as_str());
        writer.string(self.piece_type.as_str());
        writer.string(self.pack_id.as_str());
        writer.bool(self.default);
        writer.string(self.product_id.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            piece_id: reader.string(),
            piece_type: reader.string(),
            pack_id: reader.string(),
            default: reader.bool(),
            product_id: reader.string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PersonaPieceTintColour {
    pub piece_type: String,
    pub colours: Vec<String>,
}

impl PersonaPieceTintColour {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.piece_type.as_str());
        writer.u32(self.colours.len() as u32);
        self.colours.iter().for_each(|colour| writer.string(colour.as_str()));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            piece_type: reader.string(),
            colours: (0..reader.u32()).map(|_| reader.string()).collect::<Vec<String>>(),
        }
    }
}

