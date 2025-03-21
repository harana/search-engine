use harana_indexer_airbyte::indexer_airbyte::IndexerAirbyte;
use harana_indexer_application::indexer_application::IndexerApplication;
use harana_indexer_archive::indexer_archive::IndexerArchive;
use harana_indexer_audio::indexer_midi::IndexerMidi;

use harana_indexer_book::indexer_epub::IndexerEpub;
use harana_indexer_book::indexer_fb2::IndexerFb2;
use harana_indexer_book::indexer_mobi::IndexerMobi;
use harana_indexer_book::indexer_rm::IndexerRm;
use harana_indexer_book::indexer_twee::IndexerTwee;

use harana_indexer_calendar::indexer_ical::IndexerIcal;

use harana_indexer_code::indexer_comments::IndexerComments;

use harana_indexer_contact::indexer_vcard::IndexerVcard;

use harana_indexer_data::indexer_csv::IndexerCsv;
use harana_indexer_data::indexer_parquet::IndexerParquet;
use harana_indexer_data::indexer_qif::IndexerQif;

use harana_indexer_diskimage::indexer_dmg::IndexerDmg;

use harana_indexer_document::indexer_docx::IndexerDocx;
use harana_indexer_document::indexer_ipynb::IndexerIpynb;
use harana_indexer_document::indexer_md::IndexerMd;
use harana_indexer_document::indexer_pbix::IndexerPbix;
use harana_indexer_document::indexer_pdf::IndexerPdf;
use harana_indexer_document::indexer_pptx::IndexerPptx;
use harana_indexer_document::indexer_rtf::IndexerRtf;
use harana_indexer_document::indexer_txt::IndexerTxt;
use harana_indexer_document::indexer_warc::IndexerWarc;
use harana_indexer_document::indexer_xlsx::IndexerXlsx;

use harana_indexer_email::indexer_eml::IndexerEml;
use harana_indexer_email::indexer_emlx::IndexerEmlx;
use harana_indexer_email::indexer_msg::IndexerMsg;
use harana_indexer_font::indexer_font::IndexerFont;
use harana_indexer_graphics::indexer_blend::IndexerBlend;
use harana_indexer_graphics::indexer_dxf::IndexerDxf;
use harana_indexer_hardware::indexer_gds::IndexerGds;
use harana_indexer_hardware::indexer_lef::IndexerLef;
use harana_indexer_hardware::indexer_sdf::IndexerSdf;
use harana_indexer_image::indexer_imagemagick::IndexerImagemagick;
use harana_indexer_image::indexer_lrcat::IndexerLrcat;
use harana_indexer_image::indexer_psd::IndexerPsd;
use harana_indexer_image::indexer_svg::IndexerSvg;
use harana_indexer_map::indexer_fit::IndexerFit;
use harana_indexer_map::indexer_gpx::IndexerGpx;
use harana_indexer_map::indexer_kml::IndexerKml;
use harana_indexer_map::indexer_tcx::IndexerTcx;
use harana_indexer_noop::indexer_noop::IndexerNoop;
use harana_indexer_video::indexer_ffmpeg::IndexerFfmpeg;
use harana_indexer_video::indexer_srt::IndexerSrt;

pub static Airbyte: IndexerAirbyte = IndexerAirbyte;
pub static Application: IndexerApplication = IndexerApplication;

pub static Archive: IndexerArchive = IndexerArchive;

pub static Audio_Midi: IndexerMidi = IndexerMidi;

pub static Book_Epub: IndexerEpub = IndexerEpub;
pub static Book_Fb2: IndexerFb2 = IndexerFb2;
pub static Book_Mobi: IndexerMobi = IndexerMobi;
pub static Book_Rm: IndexerRm = IndexerRm;
pub static Book_Twee: IndexerTwee = IndexerTwee;

pub static Calendar_Ical: IndexerIcal = IndexerIcal;

pub static Certificate: IndexerCertificate = IndexerCertificate;

pub static Code_Comments: IndexerComments = IndexerComments;
pub static Code_XcodeProj: IndexerXcodeProj = IndexerXcodeProj;

pub static CodeArtifact_Deb: IndexerDeb = IndexerDeb;
pub static CodeArtifact_Jar: IndexerJar = IndexerJar;
pub static CodeArtifact_Msi: IndexerMsi = IndexerMsi;
pub static CodeArtifact_Rpm: IndexerRpm = IndexerRpm;
pub static CodeArtifact_Wasm: IndexerWasm = IndexerWasm;

pub static Contact_Vcard: IndexerVcard = IndexerVcard;

pub static Data_Csv: IndexerCsv = IndexerCsv;
pub static Data_Parquet: IndexerParquet = IndexerParquet;
pub static Data_Qif: IndexerQif = IndexerQif;

pub static DiskImage_Dmg: IndexerDmg = IndexerDmg;

pub static Document_Docx: IndexerDocx = IndexerDocx;
pub static Document_Ipynb: IndexerIpynb = IndexerIpynb;
pub static Document_Md: IndexerMd = IndexerMd;
pub static Document_Pbix: IndexerPbix = IndexerPbix;
pub static Document_Pdf: IndexerPdf = IndexerPdf;
pub static Document_Pptx: IndexerPptx = IndexerPptx;
pub static Document_Rtf: IndexerRtf = IndexerRtf;
pub static Document_Txt: IndexerTxt = IndexerTxt;
pub static Document_Warc: IndexerWarc = IndexerWarc;
pub static Document_Xlsx: IndexerXlsx = IndexerXlsx;

pub static Email_Eml: IndexerEml = IndexerEml;
pub static Email_Emlx: IndexerEmlx = IndexerEmlx;
pub static Email_Msg: IndexerMsg = IndexerMsg;

pub static Font: IndexerFont = IndexerFont;

pub static Graphics_Blend: IndexerBlend = IndexerBlend;
pub static Graphics_Dxf: IndexerDxf = IndexerDxf;

pub static Hardware_Gds: IndexerGds = IndexerGds;
pub static Hardware_Lef: IndexerLef = IndexerLef;
pub static Hardware_Sdf: IndexerSdf = IndexerSdf;

pub static Image_Imagemagick: IndexerImagemagick = IndexerImagemagick;
pub static Image_LrCat: IndexerLrcat = IndexerLrcat;
pub static Image_Psd: IndexerPsd = IndexerPsd;
pub static Image_Svg: IndexerSvg = IndexerSvg;

pub static Map_Fit: IndexerFit = IndexerFit;
pub static Map_Gpx: IndexerGpx = IndexerGpx;
pub static Map_Kml: IndexerKml = IndexerKml;
pub static Map_Tcx: IndexerTcx = IndexerTcx;

pub static Noop: IndexerNoop = IndexerNoop;

pub static Video_Ffmpeg: IndexerFfmpeg = IndexerFfmpeg;
pub static Video_Srt: IndexerSrt = IndexerSrt;