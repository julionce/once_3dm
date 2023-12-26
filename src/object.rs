use num_enum::TryFromPrimitive;
use once_3dm_macros::Deserialize;

use crate::{
    arc::ArcCurve,
    chunk::{self, Chunk, ChunkInStream},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    line::LineCurve,
    nurbs_curve::NurbsCurve,
    object_id::*,
    point::{Point, PointCloud},
    poly_line::PolyLineCurve,
    rollback::Rollback,
    type_code::TypeCode,
    uuid::Uuid,
};

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Kind {
    AngularDimension2Extra,
    AnnotationTextFormula,
    ArcCurve,
    BRep,
    CenterMark,
    CExampleWriteUserData,
    ClippingPlaneSurface,
    CurveOnSurface,
    CurveProxy,
    DetailView,
    DimAngular,
    DimLinear,
    DimOrdinate,
    DimRadial,
    DimStyle,
    DimStyleExtra,
    DocumentUserStringList,
    EmbeddedBitmap,
    Extrusion,
    Geometry,
    GradientColorData,
    Group,
    Hatch,
    HatchPattern,
    HistoryRecord,
    InstanceDefinition,
    InstanceRef,
    InternalObsoleteUserData,
    Layer,
    LayerExtensions,
    Leader,
    Light,
    LineCurve,
    LineType,
    Material,
    Mesh,
    MeshComponentRef,
    ModelComponent,
    ModelGeometryComponent,
    MorphControl,
    MyUserData,
    NurbsCage,
    NurbsCurve,
    NurbsSurface,
    ObjectAttributes,
    ObsoleteCCustomMeshUserData,
    ObsoleteIDefAlternativePathUserData,
    ObsoleteIDefLayerSettingsUSerData,
    ObsoleteLayerSettingsUserData,
    ObsoleteUserData,
    ObsoleteV2AnnotationArrow,
    ObsoleteV2DimAngular,
    ObsoleteV2DimLinear,
    ObsoleteV2DimRadial,
    ObsoleteV2Leader,
    ObsoleteV2TextDot,
    ObsoleteV2TextObject,
    ObsoleteV5DimAngular,
    ObsoleteV5DimExtra,
    ObsoleteV5DimLinear,
    ObsoleteV5DimOrdinate,
    ObsoleteV5DimRadial,
    ObsoleteV5HatchExtra,
    ObsoleteV5Leader,
    ObsoleteV5TextExtra,
    ObsoleteV5TextObject,
    OffsetSurface,
    PerObjectMeshParameters,
    PhysicallyBasedMaterialUserData,
    PlaneSurface,
    Point,
    PointCloud,
    PointGrid,
    PolyCurve,
    PolyEdgeCurve,
    PolyEdgeSegment,
    PolyLineCurve,
    RenderSettings,
    RevSurface,
    SubD,
    SubDComponentRef,
    SubDMeshProxyUserData,
    SumSurface,
    SurfaceProxy,
    Text,
    TextContent,
    TextDot,
    TextStyle,
    Texture,
    TextureMapping,
    UnknownUserData,
    UserStringList,
    V4V5MeshNgonUSerData,
    V5xDimStyle,
    V5BrepRegionTopologyUserData,
    V5ExtrusionDisplayMeshCache,
    V5MeshDoubleVertices,
    Viewport,
    WindowsBitmap,
    WindowsBitmapEx,
}

impl TryInto<Kind> for Uuid {
    type Error = Error;

    fn try_into(self) -> Result<Kind, Self::Error> {
        use Kind::*;
        match self {
            ANGULAR_DIMENSION2_EXTRA => Ok(AngularDimension2Extra),
            ANNOTATION_TEXT_FORMULA => Ok(AnnotationTextFormula),
            ARC_CURVE => Ok(ArcCurve),
            BREP | BREP_TL | BREP_OLD | TRIMMED_SURFACE => Ok(BRep),
            CENTER_MARK => Ok(CenterMark),
            CEXAMPLE_WRITE_USER_DATA => Ok(CExampleWriteUserData),
            CLIPPING_PLANE_SURFACE => Ok(ClippingPlaneSurface),
            CURVE_ON_SURFACE => Ok(CurveOnSurface),
            CURVE_PROXY => Ok(CurveProxy),
            DETAIL_VIEW => Ok(DetailView),
            DIM_ANGULAR => Ok(DimAngular),
            DIM_LINEAR => Ok(DimLinear),
            DIM_ORDINATE => Ok(DimOrdinate),
            DIM_RADIAL => Ok(DimRadial),
            DIM_STYLE => Ok(DimStyle),
            DIM_STYLE_EXTRA => Ok(DimStyleExtra),
            DOCUMENT_USER_STRING_LIST => Ok(DocumentUserStringList),
            EMBEDDED_BITMAP => Ok(EmbeddedBitmap),
            EXTRUSION => Ok(Extrusion),
            GEOMETRY => Ok(Geometry),
            GRADIENT_COLOR_DATA => Ok(GradientColorData),
            GROUP => Ok(Group),
            HATCH => Ok(Hatch),
            HATCH_PATTERN => Ok(HatchPattern),
            HISTORY_RECORD => Ok(HistoryRecord),
            INSTANCE_DEFINITION => Ok(InstanceDefinition),
            INSTANCE_REF => Ok(InstanceRef),
            INTERNAL_OBSOLETE_USER_DATA => Ok(InternalObsoleteUserData),
            LAYER => Ok(Layer),
            LAYER_EXTENSIONS => Ok(LayerExtensions),
            LEADER => Ok(Leader),
            LIGHT => Ok(Light),
            LINE_CURVE => Ok(LineCurve),
            LINE_TYPE => Ok(LineType),
            MATERIAL => Ok(Material),
            MESH => Ok(Mesh),
            MESH_COMPONENT_REF => Ok(MeshComponentRef),
            MODEL_COMPONENT => Ok(ModelComponent),
            MODEL_GEOMETRY_COMPONENT => Ok(ModelGeometryComponent),
            MORPH_CONTROL => Ok(MorphControl),
            MY_USER_DATA => Ok(MyUserData),
            NURBS_CAGE => Ok(NurbsCage),
            NURBS_CURVE | NURBS_CURVE_TL | NURBS_CURVE_OLD => Ok(NurbsCurve),
            NURBS_SURFACE | NURBS_SURFACE_TL | NURBS_SURFACE_OLD => Ok(NurbsSurface),
            OBJECT_ATTRIBUTES => Ok(ObjectAttributes),
            OBSOLETE_CCUSTOM_MESH_USER_DATA => Ok(ObsoleteCCustomMeshUserData),
            OBSOLETE_IDEF_ALTERNATIVE_PATH_USER_DATA => Ok(ObsoleteIDefAlternativePathUserData),
            OBSOLETE_IDEF_LAYER_SETTINGS_USER_DATA => Ok(ObsoleteIDefLayerSettingsUSerData),
            OBSOLETE_LAYER_SETTINGS_USER_DATA => Ok(ObsoleteLayerSettingsUserData),
            OBSOLETE_USER_DATA => Ok(ObsoleteUserData),
            OBSOLETE_V2_ANNOTATION_ARROW => Ok(ObsoleteV2AnnotationArrow),
            OBSOLETE_V2_DIM_ANGULAR => Ok(ObsoleteV2DimAngular),
            OBSOLETE_V2_DIM_LINEAR => Ok(ObsoleteV2DimLinear),
            OBSOLETE_V2_DIM_RADIAL => Ok(ObsoleteV2DimRadial),
            OBSOLETE_V2_LEADER => Ok(ObsoleteV2Leader),
            OBSOLETE_V2_TEXT_DOT => Ok(ObsoleteV2TextDot),
            OBSOLETE_V2_TEXT_OBJECT => Ok(ObsoleteV2TextObject),
            OBSOLETE_V5_DIM_ANGULAR => Ok(ObsoleteV5DimAngular),
            OBSOLETE_V5_DIM_EXTRA => Ok(ObsoleteV5DimExtra),
            OBSOLETE_V5_DIM_LINEAR => Ok(ObsoleteV5DimLinear),
            OBSOLETE_V5_DIM_ORDINATE => Ok(ObsoleteV5DimOrdinate),
            OBSOLETE_V5_DIM_RADIAL => Ok(ObsoleteV5DimRadial),
            OBSOLETE_V5_HATCH_EXTRA => Ok(ObsoleteV5HatchExtra),
            OBSOLETE_V5_LEADER => Ok(ObsoleteV5Leader),
            OBSOLETE_V5_TEXT_EXTRA => Ok(ObsoleteV5TextExtra),
            OBSOLETE_V5_TEXT_OBJECT => Ok(ObsoleteV5TextExtra),
            OFFSET_SURFACE => Ok(OffsetSurface),
            PER_OBJECT_MESH_PARAMETERS => Ok(PerObjectMeshParameters),
            PHYSICALLY_BASED_MATERIAL_USER_DATA => Ok(PhysicallyBasedMaterialUserData),
            PLANE_SURFACE => Ok(PlaneSurface),
            POINT => Ok(Point),
            POINT_CLOUD => Ok(PointCloud),
            POINT_GRID => Ok(PointGrid),
            POLY_CURVE | POLY_CURVE_OLD => Ok(PolyCurve),
            POLY_EDGE_CURVE => Ok(PolyEdgeCurve),
            POLY_EDGE_SEGMENT => Ok(PolyEdgeSegment),
            POLY_LINE_CURVE => Ok(PolyLineCurve),
            RENDER_SETTINGS => Ok(RenderSettings),
            REV_SURFACE | REP_SURFACE_TL => Ok(RevSurface),
            SUB_D => Ok(SubD),
            SUB_DCOMPONENT_REF => Ok(SubDComponentRef),
            SUB_DMESH_PROXY_USER_DATA => Ok(SubDMeshProxyUserData),
            SUM_SURFACE | SUM_SURFACE_TL => Ok(SumSurface),
            SURFACE_PROXY => Ok(SurfaceProxy),
            TEXT => Ok(Text),
            TEXT_CONTENT => Ok(TextContent),
            TEXT_DOT => Ok(TextDot),
            TEXT_STYLE => Ok(TextStyle),
            TEXTURE => Ok(Texture),
            TEXTURE_MAPPING => Ok(TextureMapping),
            UNKNOWN_USER_DATA => Ok(UnknownUserData),
            USER_STRING_LIST => Ok(UserStringList),
            V4_V5_MESH_NGON_USER_DATA => Ok(V4V5MeshNgonUSerData),
            V5X_DIM_STYLE => Ok(V5xDimStyle),
            V5_BREP_REGION_TOPOLOGY_USER_DATA => Ok(V5BrepRegionTopologyUserData),
            V5_EXTRUSION_DISPLAY_MESH_CACHE => Ok(V5ExtrusionDisplayMeshCache),
            V5_MESH_DOUBLE_VERTICES => Ok(V5MeshDoubleVertices),
            VIEWPORT => Ok(Viewport),
            WINDOWS_BITMAP => Ok(WindowsBitmap),
            WINDOWS_BITMAP_EX => Ok(WindowsBitmapEx),
            _ => Err(Error::Simple(ErrorKind::UnknownObjectId)),
        }
    }
}

#[derive(Default)]
pub enum Data {
    #[default]
    Empty,
    ArcCurve(ArcCurve),
    LineCurve(LineCurve),
    NurbsCurve(NurbsCurve),
    Point(Point),
    PointCloud(PointCloud),
    PolyLineCurve(PolyLineCurve),
}

#[derive(Default)]
pub struct ClassInner {
    pub uuid: Uuid,
    pub data: Data,
}

impl<V> Deserialize<V> for ClassInner
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let mut class = ClassInner::default();
        let uuid = deserialize!(
            Chunk::<{ TypeCode::OpenNurbsClassUuid as u32 }, Uuid>,
            V,
            ostream,
            "uuid"
        )
        .inner;
        class.data = match TryInto::<Kind>::try_into(uuid) {
            Ok(kind) => match kind {
                Kind::NurbsCurve => Data::NurbsCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, NurbsCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::Point => Data::Point(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, Point>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::PointCloud => Data::PointCloud(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, PointCloud>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::LineCurve => Data::LineCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, LineCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::ArcCurve => Data::ArcCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, ArcCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::PolyLineCurve => Data::PolyLineCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, PolyLineCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                _ => Data::default(),
            },
            Err(e) => {
                return Err(ErrorStack::new(e));
            }
        };
        Ok(class)
    }
}

#[derive(Default, Deserialize)]
pub struct Class {
    // TODO: move in_chunk to struct
    #[in_chunk(OpenNurbsClass)]
    pub inner: ClassInner,
}

#[derive(Default, Deserialize)]
pub struct Record {
    #[in_chunk(ObjectRecordType)]
    _empty: (),
    pub class: Class,
}

#[derive(Default)]
pub struct Table {
    pub records: Vec<Record>,
}

impl<V> Deserialize<V> for Table
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let mut records = vec![];
        loop {
            let type_code = deserialize!(Rollback<TypeCode>, V, ostream, "type_code").inner;
            match type_code {
                TypeCode::ObjectRecord => {
                    records.push(
                        deserialize!(
                            Chunk::<{ TypeCode::ObjectRecord as u32 }, Record>,
                            V,
                            ostream,
                            "record"
                        )
                        .inner,
                    );
                }
                TypeCode::EndOfTable => {
                    break;
                }
                _ => {
                    return Err(ErrorStack::new(Error::Simple(
                        ErrorKind::InvalidChunkTypeCode,
                    )));
                }
            };
        }
        Ok(Self { records })
    }
}
