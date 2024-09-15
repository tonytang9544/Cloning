pub type QualifierKey = :: string_cache :: Atom < QualifierKeyStaticSet > ;
# [derive (PartialEq , Eq , PartialOrd , Ord)] pub struct QualifierKeyStaticSet ;
impl :: string_cache :: StaticAtomSet for QualifierKeyStaticSet { fn get () -> & 'static :: string_cache :: PhfStrSet { static SET : :: string_cache :: PhfStrSet = :: string_cache :: PhfStrSet { key : 15467950696543387533u64 , disps : & [(5u32 , 11u32) , (0u32 , 0u32) , (4u32 , 6u32) , (12u32 , 30u32) , (13u32 , 27u32) , (2u32 , 0u32) , (25u32 , 21u32) , (0u32 , 33u32) , (2u32 , 25u32) , (0u32 , 36u32)] , atoms : & ["locus_tag",
"regulatory_class",
"exception",
"db_xref",
"standard_name",
"isolation_source",
"gap_type",
"gene",
"organelle",
"ncRNA_class",
"mobile_element_type",
"linkage_evidence",
"codon_start",
"",
"translation",
"sub_strain",
"anticodon",
"transl_table",
"tissue_type",
"organism",
"pseudo",
"rpt_type",
"estimated_length",
"product",
"note",
"map",
"gene_synonym",
"experiment",
"allele",
"label",
"ribosomal_slippage",
"chromosome",
"strain",
"EC_number",
"number",
"protein_id",
"transcript_id",
"function",
"mol_type",
"old_locus_tag",
"recombination_class",
"transl_except",
"nomenclature",
"bound_moiety",
"country",
"inference",
"codon_recognized"] , hashes : & [2177836023u32 , 4227444904u32 , 3935923036u32 , 2450792016u32 , 3361277847u32 , 2168988015u32 , 1994187980u32 , 204506989u32 , 3004541539u32 , 2809597890u32 , 3250452906u32 , 2515417244u32 , 4248715260u32 , 811901650u32 , 2674435430u32 , 333843063u32 , 1290144014u32 , 2294963260u32 , 1030328885u32 , 2701521110u32 , 718262908u32 , 945549777u32 , 1584595919u32 , 2857409975u32 , 2600778679u32 , 3206833967u32 , 3243193155u32 , 3548792033u32 , 4161635546u32 , 775359728u32 , 1745864180u32 , 462369687u32 , 984331342u32 , 2697988430u32 , 191428919u32 , 1022560976u32 , 2145072010u32 , 2346712105u32 , 636042323u32 , 2524862935u32 , 3917605726u32 , 3287994471u32 , 809635678u32 , 1136091494u32 , 97036694u32 , 2461382242u32 , 1360595692u32] } ;
& SET } fn empty_string_index () -> u32 { 13u32 } } pub const ATOM_QUALIFIERKEY__6C_6F_63_75_73_5F_74_61_67 : QualifierKey = QualifierKey :: pack_static (0u32) ;
pub const ATOM_QUALIFIERKEY__72_65_67_75_6C_61_74_6F_72_79_5F_63_6C_61_73_73 : QualifierKey = QualifierKey :: pack_static (1u32) ;
pub const ATOM_QUALIFIERKEY__65_78_63_65_70_74_69_6F_6E : QualifierKey = QualifierKey :: pack_static (2u32) ;
pub const ATOM_QUALIFIERKEY__64_62_5F_78_72_65_66 : QualifierKey = QualifierKey :: pack_static (3u32) ;
pub const ATOM_QUALIFIERKEY__73_74_61_6E_64_61_72_64_5F_6E_61_6D_65 : QualifierKey = QualifierKey :: pack_static (4u32) ;
pub const ATOM_QUALIFIERKEY__69_73_6F_6C_61_74_69_6F_6E_5F_73_6F_75_72_63_65 : QualifierKey = QualifierKey :: pack_static (5u32) ;
pub const ATOM_QUALIFIERKEY__67_61_70_5F_74_79_70_65 : QualifierKey = QualifierKey :: pack_static (6u32) ;
pub const ATOM_QUALIFIERKEY__67_65_6E_65 : QualifierKey = QualifierKey :: pack_static (7u32) ;
pub const ATOM_QUALIFIERKEY__6F_72_67_61_6E_65_6C_6C_65 : QualifierKey = QualifierKey :: pack_static (8u32) ;
pub const ATOM_QUALIFIERKEY__6E_63_52_4E_41_5F_63_6C_61_73_73 : QualifierKey = QualifierKey :: pack_static (9u32) ;
pub const ATOM_QUALIFIERKEY__6D_6F_62_69_6C_65_5F_65_6C_65_6D_65_6E_74_5F_74_79_70_65 : QualifierKey = QualifierKey :: pack_static (10u32) ;
pub const ATOM_QUALIFIERKEY__6C_69_6E_6B_61_67_65_5F_65_76_69_64_65_6E_63_65 : QualifierKey = QualifierKey :: pack_static (11u32) ;
pub const ATOM_QUALIFIERKEY__63_6F_64_6F_6E_5F_73_74_61_72_74 : QualifierKey = QualifierKey :: pack_static (12u32) ;
pub const ATOM_QUALIFIERKEY_ : QualifierKey = QualifierKey :: pack_static (13u32) ;
pub const ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_61_74_69_6F_6E : QualifierKey = QualifierKey :: pack_static (14u32) ;
pub const ATOM_QUALIFIERKEY__73_75_62_5F_73_74_72_61_69_6E : QualifierKey = QualifierKey :: pack_static (15u32) ;
pub const ATOM_QUALIFIERKEY__61_6E_74_69_63_6F_64_6F_6E : QualifierKey = QualifierKey :: pack_static (16u32) ;
pub const ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_5F_74_61_62_6C_65 : QualifierKey = QualifierKey :: pack_static (17u32) ;
pub const ATOM_QUALIFIERKEY__74_69_73_73_75_65_5F_74_79_70_65 : QualifierKey = QualifierKey :: pack_static (18u32) ;
pub const ATOM_QUALIFIERKEY__6F_72_67_61_6E_69_73_6D : QualifierKey = QualifierKey :: pack_static (19u32) ;
pub const ATOM_QUALIFIERKEY__70_73_65_75_64_6F : QualifierKey = QualifierKey :: pack_static (20u32) ;
pub const ATOM_QUALIFIERKEY__72_70_74_5F_74_79_70_65 : QualifierKey = QualifierKey :: pack_static (21u32) ;
pub const ATOM_QUALIFIERKEY__65_73_74_69_6D_61_74_65_64_5F_6C_65_6E_67_74_68 : QualifierKey = QualifierKey :: pack_static (22u32) ;
pub const ATOM_QUALIFIERKEY__70_72_6F_64_75_63_74 : QualifierKey = QualifierKey :: pack_static (23u32) ;
pub const ATOM_QUALIFIERKEY__6E_6F_74_65 : QualifierKey = QualifierKey :: pack_static (24u32) ;
pub const ATOM_QUALIFIERKEY__6D_61_70 : QualifierKey = QualifierKey :: pack_static (25u32) ;
pub const ATOM_QUALIFIERKEY__67_65_6E_65_5F_73_79_6E_6F_6E_79_6D : QualifierKey = QualifierKey :: pack_static (26u32) ;
pub const ATOM_QUALIFIERKEY__65_78_70_65_72_69_6D_65_6E_74 : QualifierKey = QualifierKey :: pack_static (27u32) ;
pub const ATOM_QUALIFIERKEY__61_6C_6C_65_6C_65 : QualifierKey = QualifierKey :: pack_static (28u32) ;
pub const ATOM_QUALIFIERKEY__6C_61_62_65_6C : QualifierKey = QualifierKey :: pack_static (29u32) ;
pub const ATOM_QUALIFIERKEY__72_69_62_6F_73_6F_6D_61_6C_5F_73_6C_69_70_70_61_67_65 : QualifierKey = QualifierKey :: pack_static (30u32) ;
pub const ATOM_QUALIFIERKEY__63_68_72_6F_6D_6F_73_6F_6D_65 : QualifierKey = QualifierKey :: pack_static (31u32) ;
pub const ATOM_QUALIFIERKEY__73_74_72_61_69_6E : QualifierKey = QualifierKey :: pack_static (32u32) ;
pub const ATOM_QUALIFIERKEY__45_43_5F_6E_75_6D_62_65_72 : QualifierKey = QualifierKey :: pack_static (33u32) ;
pub const ATOM_QUALIFIERKEY__6E_75_6D_62_65_72 : QualifierKey = QualifierKey :: pack_static (34u32) ;
pub const ATOM_QUALIFIERKEY__70_72_6F_74_65_69_6E_5F_69_64 : QualifierKey = QualifierKey :: pack_static (35u32) ;
pub const ATOM_QUALIFIERKEY__74_72_61_6E_73_63_72_69_70_74_5F_69_64 : QualifierKey = QualifierKey :: pack_static (36u32) ;
pub const ATOM_QUALIFIERKEY__66_75_6E_63_74_69_6F_6E : QualifierKey = QualifierKey :: pack_static (37u32) ;
pub const ATOM_QUALIFIERKEY__6D_6F_6C_5F_74_79_70_65 : QualifierKey = QualifierKey :: pack_static (38u32) ;
pub const ATOM_QUALIFIERKEY__6F_6C_64_5F_6C_6F_63_75_73_5F_74_61_67 : QualifierKey = QualifierKey :: pack_static (39u32) ;
pub const ATOM_QUALIFIERKEY__72_65_63_6F_6D_62_69_6E_61_74_69_6F_6E_5F_63_6C_61_73_73 : QualifierKey = QualifierKey :: pack_static (40u32) ;
pub const ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_5F_65_78_63_65_70_74 : QualifierKey = QualifierKey :: pack_static (41u32) ;
pub const ATOM_QUALIFIERKEY__6E_6F_6D_65_6E_63_6C_61_74_75_72_65 : QualifierKey = QualifierKey :: pack_static (42u32) ;
pub const ATOM_QUALIFIERKEY__62_6F_75_6E_64_5F_6D_6F_69_65_74_79 : QualifierKey = QualifierKey :: pack_static (43u32) ;
pub const ATOM_QUALIFIERKEY__63_6F_75_6E_74_72_79 : QualifierKey = QualifierKey :: pack_static (44u32) ;
pub const ATOM_QUALIFIERKEY__69_6E_66_65_72_65_6E_63_65 : QualifierKey = QualifierKey :: pack_static (45u32) ;
pub const ATOM_QUALIFIERKEY__63_6F_64_6F_6E_5F_72_65_63_6F_67_6E_69_7A_65_64 : QualifierKey = QualifierKey :: pack_static (46u32) ;
# [macro_export] macro_rules ! qualifier_key { ("locus_tag") => { $ crate :: ATOM_QUALIFIERKEY__6C_6F_63_75_73_5F_74_61_67 } ;
("regulatory_class") => { $ crate :: ATOM_QUALIFIERKEY__72_65_67_75_6C_61_74_6F_72_79_5F_63_6C_61_73_73 } ;
("exception") => { $ crate :: ATOM_QUALIFIERKEY__65_78_63_65_70_74_69_6F_6E } ;
("db_xref") => { $ crate :: ATOM_QUALIFIERKEY__64_62_5F_78_72_65_66 } ;
("standard_name") => { $ crate :: ATOM_QUALIFIERKEY__73_74_61_6E_64_61_72_64_5F_6E_61_6D_65 } ;
("isolation_source") => { $ crate :: ATOM_QUALIFIERKEY__69_73_6F_6C_61_74_69_6F_6E_5F_73_6F_75_72_63_65 } ;
("gap_type") => { $ crate :: ATOM_QUALIFIERKEY__67_61_70_5F_74_79_70_65 } ;
("gene") => { $ crate :: ATOM_QUALIFIERKEY__67_65_6E_65 } ;
("organelle") => { $ crate :: ATOM_QUALIFIERKEY__6F_72_67_61_6E_65_6C_6C_65 } ;
("ncRNA_class") => { $ crate :: ATOM_QUALIFIERKEY__6E_63_52_4E_41_5F_63_6C_61_73_73 } ;
("mobile_element_type") => { $ crate :: ATOM_QUALIFIERKEY__6D_6F_62_69_6C_65_5F_65_6C_65_6D_65_6E_74_5F_74_79_70_65 } ;
("linkage_evidence") => { $ crate :: ATOM_QUALIFIERKEY__6C_69_6E_6B_61_67_65_5F_65_76_69_64_65_6E_63_65 } ;
("codon_start") => { $ crate :: ATOM_QUALIFIERKEY__63_6F_64_6F_6E_5F_73_74_61_72_74 } ;
("") => { $ crate :: ATOM_QUALIFIERKEY_ } ;
("translation") => { $ crate :: ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_61_74_69_6F_6E } ;
("sub_strain") => { $ crate :: ATOM_QUALIFIERKEY__73_75_62_5F_73_74_72_61_69_6E } ;
("anticodon") => { $ crate :: ATOM_QUALIFIERKEY__61_6E_74_69_63_6F_64_6F_6E } ;
("transl_table") => { $ crate :: ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_5F_74_61_62_6C_65 } ;
("tissue_type") => { $ crate :: ATOM_QUALIFIERKEY__74_69_73_73_75_65_5F_74_79_70_65 } ;
("organism") => { $ crate :: ATOM_QUALIFIERKEY__6F_72_67_61_6E_69_73_6D } ;
("pseudo") => { $ crate :: ATOM_QUALIFIERKEY__70_73_65_75_64_6F } ;
("rpt_type") => { $ crate :: ATOM_QUALIFIERKEY__72_70_74_5F_74_79_70_65 } ;
("estimated_length") => { $ crate :: ATOM_QUALIFIERKEY__65_73_74_69_6D_61_74_65_64_5F_6C_65_6E_67_74_68 } ;
("product") => { $ crate :: ATOM_QUALIFIERKEY__70_72_6F_64_75_63_74 } ;
("note") => { $ crate :: ATOM_QUALIFIERKEY__6E_6F_74_65 } ;
("map") => { $ crate :: ATOM_QUALIFIERKEY__6D_61_70 } ;
("gene_synonym") => { $ crate :: ATOM_QUALIFIERKEY__67_65_6E_65_5F_73_79_6E_6F_6E_79_6D } ;
("experiment") => { $ crate :: ATOM_QUALIFIERKEY__65_78_70_65_72_69_6D_65_6E_74 } ;
("allele") => { $ crate :: ATOM_QUALIFIERKEY__61_6C_6C_65_6C_65 } ;
("label") => { $ crate :: ATOM_QUALIFIERKEY__6C_61_62_65_6C } ;
("ribosomal_slippage") => { $ crate :: ATOM_QUALIFIERKEY__72_69_62_6F_73_6F_6D_61_6C_5F_73_6C_69_70_70_61_67_65 } ;
("chromosome") => { $ crate :: ATOM_QUALIFIERKEY__63_68_72_6F_6D_6F_73_6F_6D_65 } ;
("strain") => { $ crate :: ATOM_QUALIFIERKEY__73_74_72_61_69_6E } ;
("EC_number") => { $ crate :: ATOM_QUALIFIERKEY__45_43_5F_6E_75_6D_62_65_72 } ;
("number") => { $ crate :: ATOM_QUALIFIERKEY__6E_75_6D_62_65_72 } ;
("protein_id") => { $ crate :: ATOM_QUALIFIERKEY__70_72_6F_74_65_69_6E_5F_69_64 } ;
("transcript_id") => { $ crate :: ATOM_QUALIFIERKEY__74_72_61_6E_73_63_72_69_70_74_5F_69_64 } ;
("function") => { $ crate :: ATOM_QUALIFIERKEY__66_75_6E_63_74_69_6F_6E } ;
("mol_type") => { $ crate :: ATOM_QUALIFIERKEY__6D_6F_6C_5F_74_79_70_65 } ;
("old_locus_tag") => { $ crate :: ATOM_QUALIFIERKEY__6F_6C_64_5F_6C_6F_63_75_73_5F_74_61_67 } ;
("recombination_class") => { $ crate :: ATOM_QUALIFIERKEY__72_65_63_6F_6D_62_69_6E_61_74_69_6F_6E_5F_63_6C_61_73_73 } ;
("transl_except") => { $ crate :: ATOM_QUALIFIERKEY__74_72_61_6E_73_6C_5F_65_78_63_65_70_74 } ;
("nomenclature") => { $ crate :: ATOM_QUALIFIERKEY__6E_6F_6D_65_6E_63_6C_61_74_75_72_65 } ;
("bound_moiety") => { $ crate :: ATOM_QUALIFIERKEY__62_6F_75_6E_64_5F_6D_6F_69_65_74_79 } ;
("country") => { $ crate :: ATOM_QUALIFIERKEY__63_6F_75_6E_74_72_79 } ;
("inference") => { $ crate :: ATOM_QUALIFIERKEY__69_6E_66_65_72_65_6E_63_65 } ;
("codon_recognized") => { $ crate :: ATOM_QUALIFIERKEY__63_6F_64_6F_6E_5F_72_65_63_6F_67_6E_69_7A_65_64 } ;
}pub type FeatureKind = :: string_cache :: Atom < FeatureKindStaticSet > ;
# [derive (PartialEq , Eq , PartialOrd , Ord)] pub struct FeatureKindStaticSet ;
impl :: string_cache :: StaticAtomSet for FeatureKindStaticSet { fn get () -> & 'static :: string_cache :: PhfStrSet { static SET : :: string_cache :: PhfStrSet = :: string_cache :: PhfStrSet { key : 15467950696543387533u64 , disps : & [(2u32 , 0u32) , (0u32 , 0u32) , (13u32 , 18u32) , (1u32 , 21u32) , (1u32 , 0u32) , (1u32 , 24u32)] , atoms : & ["rep_origin",
"D_segment",
"C_region",
"tRNA",
"exon",
"mRNA",
"misc_RNA",
"ncRNA",
"protein_bind",
"V_segment",
"gene",
"misc_recomb",
"J_segment",
"",
"CDS",
"mobile_element",
"source",
"regulatory",
"misc_feature",
"D-loop",
"repeat_region",
"precursor_RNA",
"centromere",
"rRNA",
"tmRNA",
"assembly_gap"] , hashes : & [3384762065u32 , 3788220913u32 , 4240860826u32 , 16501275u32 , 3849330554u32 , 3308325784u32 , 137191317u32 , 697905356u32 , 1794267211u32 , 3487687027u32 , 204506989u32 , 554550019u32 , 3489482566u32 , 811901650u32 , 2922173005u32 , 1204799909u32 , 2365174978u32 , 1653748757u32 , 2519401310u32 , 2100217464u32 , 3812890511u32 , 1445520087u32 , 162770819u32 , 4117245439u32 , 2129042454u32 , 837526089u32] } ;
& SET } fn empty_string_index () -> u32 { 13u32 } } pub const ATOM_FEATUREKIND__72_65_70_5F_6F_72_69_67_69_6E : FeatureKind = FeatureKind :: pack_static (0u32) ;
pub const ATOM_FEATUREKIND__44_5F_73_65_67_6D_65_6E_74 : FeatureKind = FeatureKind :: pack_static (1u32) ;
pub const ATOM_FEATUREKIND__43_5F_72_65_67_69_6F_6E : FeatureKind = FeatureKind :: pack_static (2u32) ;
pub const ATOM_FEATUREKIND__74_52_4E_41 : FeatureKind = FeatureKind :: pack_static (3u32) ;
pub const ATOM_FEATUREKIND__65_78_6F_6E : FeatureKind = FeatureKind :: pack_static (4u32) ;
pub const ATOM_FEATUREKIND__6D_52_4E_41 : FeatureKind = FeatureKind :: pack_static (5u32) ;
pub const ATOM_FEATUREKIND__6D_69_73_63_5F_52_4E_41 : FeatureKind = FeatureKind :: pack_static (6u32) ;
pub const ATOM_FEATUREKIND__6E_63_52_4E_41 : FeatureKind = FeatureKind :: pack_static (7u32) ;
pub const ATOM_FEATUREKIND__70_72_6F_74_65_69_6E_5F_62_69_6E_64 : FeatureKind = FeatureKind :: pack_static (8u32) ;
pub const ATOM_FEATUREKIND__56_5F_73_65_67_6D_65_6E_74 : FeatureKind = FeatureKind :: pack_static (9u32) ;
pub const ATOM_FEATUREKIND__67_65_6E_65 : FeatureKind = FeatureKind :: pack_static (10u32) ;
pub const ATOM_FEATUREKIND__6D_69_73_63_5F_72_65_63_6F_6D_62 : FeatureKind = FeatureKind :: pack_static (11u32) ;
pub const ATOM_FEATUREKIND__4A_5F_73_65_67_6D_65_6E_74 : FeatureKind = FeatureKind :: pack_static (12u32) ;
pub const ATOM_FEATUREKIND_ : FeatureKind = FeatureKind :: pack_static (13u32) ;
pub const ATOM_FEATUREKIND__43_44_53 : FeatureKind = FeatureKind :: pack_static (14u32) ;
pub const ATOM_FEATUREKIND__6D_6F_62_69_6C_65_5F_65_6C_65_6D_65_6E_74 : FeatureKind = FeatureKind :: pack_static (15u32) ;
pub const ATOM_FEATUREKIND__73_6F_75_72_63_65 : FeatureKind = FeatureKind :: pack_static (16u32) ;
pub const ATOM_FEATUREKIND__72_65_67_75_6C_61_74_6F_72_79 : FeatureKind = FeatureKind :: pack_static (17u32) ;
pub const ATOM_FEATUREKIND__6D_69_73_63_5F_66_65_61_74_75_72_65 : FeatureKind = FeatureKind :: pack_static (18u32) ;
pub const ATOM_FEATUREKIND__44_2D_6C_6F_6F_70 : FeatureKind = FeatureKind :: pack_static (19u32) ;
pub const ATOM_FEATUREKIND__72_65_70_65_61_74_5F_72_65_67_69_6F_6E : FeatureKind = FeatureKind :: pack_static (20u32) ;
pub const ATOM_FEATUREKIND__70_72_65_63_75_72_73_6F_72_5F_52_4E_41 : FeatureKind = FeatureKind :: pack_static (21u32) ;
pub const ATOM_FEATUREKIND__63_65_6E_74_72_6F_6D_65_72_65 : FeatureKind = FeatureKind :: pack_static (22u32) ;
pub const ATOM_FEATUREKIND__72_52_4E_41 : FeatureKind = FeatureKind :: pack_static (23u32) ;
pub const ATOM_FEATUREKIND__74_6D_52_4E_41 : FeatureKind = FeatureKind :: pack_static (24u32) ;
pub const ATOM_FEATUREKIND__61_73_73_65_6D_62_6C_79_5F_67_61_70 : FeatureKind = FeatureKind :: pack_static (25u32) ;
# [macro_export] macro_rules ! feature_kind { ("rep_origin") => { $ crate :: ATOM_FEATUREKIND__72_65_70_5F_6F_72_69_67_69_6E } ;
("D_segment") => { $ crate :: ATOM_FEATUREKIND__44_5F_73_65_67_6D_65_6E_74 } ;
("C_region") => { $ crate :: ATOM_FEATUREKIND__43_5F_72_65_67_69_6F_6E } ;
("tRNA") => { $ crate :: ATOM_FEATUREKIND__74_52_4E_41 } ;
("exon") => { $ crate :: ATOM_FEATUREKIND__65_78_6F_6E } ;
("mRNA") => { $ crate :: ATOM_FEATUREKIND__6D_52_4E_41 } ;
("misc_RNA") => { $ crate :: ATOM_FEATUREKIND__6D_69_73_63_5F_52_4E_41 } ;
("ncRNA") => { $ crate :: ATOM_FEATUREKIND__6E_63_52_4E_41 } ;
("protein_bind") => { $ crate :: ATOM_FEATUREKIND__70_72_6F_74_65_69_6E_5F_62_69_6E_64 } ;
("V_segment") => { $ crate :: ATOM_FEATUREKIND__56_5F_73_65_67_6D_65_6E_74 } ;
("gene") => { $ crate :: ATOM_FEATUREKIND__67_65_6E_65 } ;
("misc_recomb") => { $ crate :: ATOM_FEATUREKIND__6D_69_73_63_5F_72_65_63_6F_6D_62 } ;
("J_segment") => { $ crate :: ATOM_FEATUREKIND__4A_5F_73_65_67_6D_65_6E_74 } ;
("") => { $ crate :: ATOM_FEATUREKIND_ } ;
("CDS") => { $ crate :: ATOM_FEATUREKIND__43_44_53 } ;
("mobile_element") => { $ crate :: ATOM_FEATUREKIND__6D_6F_62_69_6C_65_5F_65_6C_65_6D_65_6E_74 } ;
("source") => { $ crate :: ATOM_FEATUREKIND__73_6F_75_72_63_65 } ;
("regulatory") => { $ crate :: ATOM_FEATUREKIND__72_65_67_75_6C_61_74_6F_72_79 } ;
("misc_feature") => { $ crate :: ATOM_FEATUREKIND__6D_69_73_63_5F_66_65_61_74_75_72_65 } ;
("D-loop") => { $ crate :: ATOM_FEATUREKIND__44_2D_6C_6F_6F_70 } ;
("repeat_region") => { $ crate :: ATOM_FEATUREKIND__72_65_70_65_61_74_5F_72_65_67_69_6F_6E } ;
("precursor_RNA") => { $ crate :: ATOM_FEATUREKIND__70_72_65_63_75_72_73_6F_72_5F_52_4E_41 } ;
("centromere") => { $ crate :: ATOM_FEATUREKIND__63_65_6E_74_72_6F_6D_65_72_65 } ;
("rRNA") => { $ crate :: ATOM_FEATUREKIND__72_52_4E_41 } ;
("tmRNA") => { $ crate :: ATOM_FEATUREKIND__74_6D_52_4E_41 } ;
("assembly_gap") => { $ crate :: ATOM_FEATUREKIND__61_73_73_65_6D_62_6C_79_5F_67_61_70 } ;
}