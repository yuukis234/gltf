
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::ops::Deref;
use {json, texture, Gltf};

/// The alpha rendering mode of a material.
pub enum AlphaMode {
    Blend,
    Mask,
    Opaque,
}

///  The material appearance of a primitive.
pub struct Material {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::material::Material,
}

impl Material {
    /// Constructs a `Material`.
    pub fn new(gltf: &'a Gltf, json: &'a json::material::Material) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::material::Material {
        self.json
    }

    ///  The alpha cutoff value of the material.
    pub fn alpha_cutoff(&self) -> f32 {
        self.json.alpha_cutoff.0
    }

    /// The alpha rendering mode of the material.  The material's alpha rendering
    /// mode enumeration specifying the interpretation of the alpha value of the main
    /// factor and texture.  * In `Opaque` mode (default) the alpha value is ignored
    /// and the rendered output is fully opaque.  * In `Mask` mode, the rendered
    /// output is either fully opaque or fully transparent depending on the alpha
    /// value and the specified alpha cutoff value.  * In `Blend` mode, the alpha
    /// value is used to composite the source and destination areas and the rendered
    /// output is combined with the background using the normal painting operation
    /// (i.e. the Porter and Duff over operIterator).
    pub fn alpha_mode(&self) -> AlphaMode {
        use self::AlphaMode::*;
        match self.json.alpha_mode.0.as_ref() {
            "BLEND" => Blend,
            "MASK" => Mask,
            "OPAQUE" => Opaque,
            _ => unreachable!(),
        }
    }

    ///  Specifies whether the material is double-sided.  * When this value is false,
    /// back-face culling is enabled.  * When this value is true, back-face culling
    /// is disabled and double sided lighting is enabled.  The back-face must have
    /// its normals reversed before the lighting equation is evaluated.
    pub fn double_sided(&self) -> bool {
        self.json.double_sided
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    ///  A set of parameter values that are used to define the metallic-roughness
    /// material model from Physically-Based Rendering (PBR) methodology. When not
    /// specified, all the default values of `pbrMetallicRoughness` apply.
    pub fn pbr_metallic_roughness(&self) -> Option<PbrMetallicRoughness> {
        self.json.pbr_metallic_roughness.as_ref().map(|json| {
            PbrMetallicRoughness::new(self.gltf, json)
        })
    }

    ///  A tangent space normal map. The texture contains RGB components in linear
    /// space. Each texel represents the XYZ components of a normal vector in tangent
    /// space. Red [0 to 255] maps to X [-1 to 1]. Green [0 to 255] maps to Y
    /// [-1 to 1]. Blue [128 to 255] maps to Z [1/255 to 1]. The normal vectors use
    /// OpenGL conventions where +X is right and +Y is up. +Z points toward the
    /// viewer.
    pub fn normal_texture(&self) -> Option<NormalTexture> {
        self.json.normal_texture.as_ref().map(|json| {
            let texture = self.gltf.textures().nth(json.index.value()).unwrap();
            NormalTexture::new(texture, json)
        })
    }

    ///  The occlusion map texture. The occlusion values are sampled from the R
    /// channel. Higher values indicate areas that should receive full indirect
    /// lighting and lower values indicate no indirect lighting. These values are
    /// linear. If other channels are present (GBA), they are ignored for occlusion
    /// calculations.
    pub fn occlusion_texture(&self) -> Option<OcclusionTexture> {
        self.json.occlusion_texture.as_ref().map(|json| {
            let texture = self.gltf.textures().nth(json.index.value()).unwrap();
            OcclusionTexture::new(texture, json)
        })
    }

    ///  The emissive map controls the color and intensity of the light being emitted
    /// by the material. This texture contains RGB components in sRGB color space. If
    /// a fourth component (A) is present, it is ignored.
    pub fn emissive_texture(&self) -> Option<texture::Info> {
        self.json.emissive_texture.as_ref().map(|json| {
            let texture = self.gltf.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    ///  The emissive color of the material.
    pub fn emissive_factor(&self) -> [f32; 3] {
        self.json.emissive_factor.0
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::material::MaterialExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
///  A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
pub struct PbrMetallicRoughness {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::material::PbrMetallicRoughness,
}

impl PbrMetallicRoughness {
    /// Constructs a `PbrMetallicRoughness`.
    pub fn new(
        gltf: &'a Gltf,
        json: &'a json::material::PbrMetallicRoughness,
    ) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::material::PbrMetallicRoughness {
        self.json
    }

    ///  The material's base color factor.
    pub fn base_color_factor(&self) -> [f32; 4] {
        self.json.base_color_factor.0
    }

    ///  The base color texture.
    pub fn base_color_texture(&self) -> Option<texture::Info> {
        self.json.base_color_texture.as_ref().map(|json| {
            let texture = self.gltf.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    ///  The metalness of the material.
    pub fn metallic_factor(&self) -> f32 {
        self.json.metallic_factor.0
    }

    ///  The roughness of the material.  * A value of 1.0 means the material is
    /// completely rough. * A value of 0.0 means the material is completely smooth.
    pub fn roughness_factor(&self) -> f32 {
        self.json.roughness_factor.0
    }

    ///  The metallic-roughness texture.  This texture has two components:  * The
    /// first component (R) contains the metallic-ness of the material. * The second
    /// component (G) contains the roughness of the material. * If the third
    /// component (B) and/or the fourth component (A) are present then they are
    /// ignored.
    pub fn metallic_roughness_texture(&self) -> Option<texture::Info> {
        self.json.metallic_roughness_texture.as_ref().map(|json| {
            let texture = self.gltf.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::material::PbrMetallicRoughnessExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

///  Defines the normal texture of a material.
pub struct NormalTexture {
    /// The parent `Texture` struct.
    texture: texture::Texture,

    /// The corresponding JSON struct.
    json: &'a json::material::NormalTexture,
}

impl NormalTexture {
    /// Constructs a `NormalTexture`.
    pub fn new(
        texture: texture::Texture,
        json: &'a json::material::NormalTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::material::NormalTexture {
        self.json
    }

    /// The scalar multiplier applied to each normal vector of the texture.
    /// This value is ignored if normalTexture is not specified.
    pub fn scale(&self) -> f32 {
        self.json.scale
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::material::NormalTextureExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
///  Defines the occlusion texture of a material.
pub struct OcclusionTexture {
    /// The parent `Texture` struct.
    texture: texture::Texture,

    /// The corresponding JSON struct.
    json: &'a json::material::OcclusionTexture,
}

impl OcclusionTexture {
    /// Constructs a `OcclusionTexture`.
    pub fn new(
        texture: texture::Texture,
        json: &'a json::material::OcclusionTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::material::OcclusionTexture {
        self.json
    }

    /// The scalar multiplier controlling the amount of occlusion applied.
    pub fn strength(&self) -> f32 {
        self.json.strength.0
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::material::OcclusionTextureExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl Deref for NormalTexture {
    type Target = texture::Texture;
    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl Deref for OcclusionTexture {
    type Target = texture::Texture;
    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}


