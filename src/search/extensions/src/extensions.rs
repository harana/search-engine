use crate::extensions;
use harana_common::file_format;
use paste::paste;

extensions! {
    extension = "app"
    title = "Application"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ArbitraryBinaryData
    extension = "bin"
    title = "Arbitrary Binary Data"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MsDosExecutable
    extension = "exe"
    title = "MS-DOS Executable"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LinearExecutable
    extension = "le"
    title = "Linear Executable"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SevenZip
    extension = "7z"
    title = "7-Zip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UnixArchiver
    extension = "a"
    title = "UNIX archiver"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdvancedCompressionEngine
    extension = "ace"
    title = "Advanced Compression Engine"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Alz
    extension = "alz"
    title = "ALZip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ArchivedByRobertJung
    extension = "ar"
    title = "Archived by Robert Jung"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Bzip
    extension = "bz"
    title = "Bzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Bzip2
    extension = "bz2"
    title = "Bzip2"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Bzip3
    extension = "bz3"
    title = "Bzip2"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Cabinet
    extension = "cab"
    title = "Cabinet"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "compress"
    title = "Unix Compress"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Cpio
    extension = "cpio"
    title = "CPIO"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Gzip
    extension = "gz"
    title = "Gzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Lha
    extension = "lha"
    title = "LHA"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LempelZivFiniteStateEntropy
    extension = "lzfse"
    title = "Lempel-Ziv Finite State Entropy"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Lzip
    extension = "lz"
    title = "Lzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Lzip
    extension = "lzh"
    title = "Lziph"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Lz4
    extension = "lz4"
    title = "LZ4"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LempelZivMarkovChainAlgorithm
    extension = "lzma"
    title = "Lempel-Ziv-Markov chain algorithm"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Lzop
    extension = "lzo"
    title = "Lzop"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Larc
    extension = "lzs"
    title = "LArc"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LongRangeZip
    extension = "lrzip"
    title = "Long Range Zip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mtree"
    title = "Mtree"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pax"
    title = "Pax"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Pmarc
    extension = "pma"
    title = "PMarc"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = RoshalArchive
    extension = "rar"
    title = "Roshal Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Rzip
    extension = "rz"
    title = "Rzip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Seqbox
    extension = "sbx"
    title = "SeqBox"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "shar"
    title = "Shar"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Snappy
    extension = "sz"
    title = "Snappy"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Stuffit
    extension = "sit"
    title = "StuffIt"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = StuffitX
    extension = "sitx"
    title = "StuffIt X"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Squashfs
    extension = "squashfs"
    title = "Squashfs"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = TapeArchive
    extension = "tar"
    title = "Tape Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ExtensibleArchive
    extension = "xar"
    title = "Extensible Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Xz
    extension = "xz"
    title = "Xz"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "warc"
    title = "Warc"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsImagingFormat
    extension = "wim"
    title = "Windows Imaging Format"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UnixCompress
    extension = "Z"
    title = "UNIX compress"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Zip
    extension = "zip"
    title = "Zip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Zoo
    extension = "zoo"
    title = "Zoo"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Zpaq
    extension = "zpaq"
    title = "ZPAQ"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Zstandard
    extension = "zstd"
    title = "Zstandard"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Iff8BitSampledVoice
    extension = "8svx"
    title = "IFF 8-Bit Sampled Voice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdvancedAudioCoding
    extension = "aac"
    title = "Advanced Audio Coding"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AudioCodec3
    extension = "ac3"
    title = "Audio Codec 3"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AudioInterchangeFileFormat
    extension = "aiff"
    title = "Audio Interchange File Format"
    category = Audio
    indexer = Noop
    thumbnailer = Aiff
    opener = Noop
    viewer = Noop
    cards = []

    extension = "alac"
    title = "Apple Lossless Audio Codec"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdaptiveMultiRate
    extension = "amr"
    title = "Adaptive Multi-Rate"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MonkeysAudio
    extension = "ape"
    title = "Monkey's Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "at3"
    title = "Sony UMD Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Au
    extension = "au"
    title = "Audio Unit"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AudioVisualResearch
    extension = "avr"
    title = "Audio Visual Research"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CdAudio
    extension = "cda"
    title = "CD Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SonyDsdStreamFile
    extension = "dsf"
    title = "Sony DSD Stream File"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeFlashPlayerAudio
    extension = "f4a"
    title = "Adobe Flash Player Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeFlashPlayerAudiobook
    extension = "f4b"
    title = "Adobe Flash Player Audiobook"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = FreeLosslessAudioCodec
    extension = "flac"
    title = "Free Lossless Audio Codec"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ImpulseTrackerModule
    extension = "it"
    title = "Impulse Tracker Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "lrc"
    title = "Lyrics"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleItunesAudio
    extension = "m4a"
    title = "Apple Music Audio"
    category = Audio
    indexer = Noop
    thumbnailer = M4a
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleItunesAudiobook
    extension = "m4b"
    title = "Apple Music Audiobook"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleItunesProtectedAudio
    extension = "m4p"
    title = "Apple Music Protected Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MatroskaAudio
    extension = "mka"
    title = "Matroska Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MusicalInstrumentDigitalInterface
    extension = "midi"
    title = "Musical Instrument Digital Interface"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mmf"
    title = "Yamaha Synthetic Music"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UltimateSoundtrackerModule
    extension = "mod"
    title = "Ultimate Soundtracker Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mp1"
    title = "MPEG-1/2 Audio Layer 1"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Mpeg12AudioLayer2
    extension = "mp2"
    title = "MPEG-1/2 Audio Layer 2"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Mpeg12AudioLayer3
    extension = "mp3"
    title = "MPEG-1/2 Audio Layer 3"
    category = Audio
    indexer = Noop
    thumbnailer = Mp3
    opener = Noop
    viewer = Noop
    cards = []

    format = Musepack
    extension = "mpc"
    title = "Musepack"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mus"
    title = "Music Time Internal"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Musicxml
    extension = "musicxml"
    title = "MusicXML"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MusicxmlZipped
    extension = "mxl"
    title = "MusicXML Zipped"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ncm"
    title = "Netease Cloud Music"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ncw"
    title = "Native Instruments Compressed Wave"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OggFlac
    extension = "oga"
    title = "Ogg FLAC"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OggVorbis
    extension = "ogg"
    title = "Ogg Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OggMultiplexedMedia
    extension = "ogx"
    title = "Ogg Multiplexed Media"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OggOpus
    extension = "opus"
    title = "Ogg Opus"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "psf"
    title = "Parameter Storage Format"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = QualcommPurevoice
    extension = "qcp"
    title = "Qualcomm PureVoice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = QuiteOkAudio
    extension = "qoa"
    title = "Quite OK Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Realaudio
    extension = "ra"
    title = "RealAudio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ScreamTracker3Module
    extension = "s3m"
    title = "Scream Tracker 3 Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Soundfont2
    extension = "sf2"
    title = "SoundFont 2"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sf3"
    title = "SoundFont 3"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sfz"
    title = "SoundFont Z"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OggSpeex
    extension = "spx"
    title = "Ogg Speex"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Fasttracker2ExtendedModule
    extension = "xm"
    title = "FastTracker 2 Extended Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CreativeVoice
    extension = "voc"
    title = "Creative Voice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WaveformAudio
    extension = "wav"
    title = "Waveform Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Wav
    opener = Noop
    viewer = Noop
    cards = []

    format = Wavpack
    extension = "wv"
    title = "WavPack"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsMediaAudio
    extension = "wma"
    title = "Windows Media Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "awz"
    title = "Amazon Kindle Book"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "awz3"
    title = "Amazon Kindle Book"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = BroadBandEbook
    extension = "bbeb"
    title = "Broad Band eBook"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ElectronicPublication
    extension = "epub"
    title = "Electronic Publication"
    category = Book
    indexer = Book_Epub
    thumbnailer = Noop
    opener = Noop
    viewer = Epub
    cards = []

    format = Fictionbook
    extension = "fb2"
    title = "FictionBook"
    category = Book
    indexer = Book_Fb2
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = FictionbookZipped
    extension = "fbz"
    title = "FictionBook Zipped"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftReader
    extension = "lit"
    title = "Microsoft Reader"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Mobipocket
    extension = "mobi"
    title = "Mobipocket"
    category = Book
    indexer = Book_Mobi
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ogam"
    title = "OGAM Markup Language"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "rm"
    title = "Remarkable"
    category = Book
    indexer = Book_Rm
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tw"
    title = "Twine Story"
    category = Book
    indexer = Book_Twee
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "twee"
    title = "Twine Story"
    category = Book
    indexer = Book_Twee
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "usfm"
    title = "Unified Standard Format Marker"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "zim"
    title = "Zim Wiki Extract"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Icalendar
    extension = "ics"
    title = "iCalendar"
    category = Calendar
    indexer = Calendar_Ical
    thumbnailer = Noop
    opener = Noop
    viewer = Calendar
    cards = []

    extension = "vcf"
    title = "Variant Call Format"
    category = Contact
    indexer = Contact_Vcard
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PgpMessage
    extension = "asc"
    title = "PGP Message"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PgpPrivateKeyBlock
    extension = "asc"
    title = "PGP Key Block"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PgpSignature
    extension = "asc"
    title = "PGP Signature"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = PgpSignedMessage
    extension = "asc"
    title = "PGP Signed Message"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = DerCertificate
    extension = "der"
    title = "DER Certificate"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = JavaKeystore
    extension = "jks"
    title = "Java KeyStore"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = PemCertificate
    extension = "crt"
    title = "PEM Certificate"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = PemCertificateSigningRequest
    extension = "crt"
    title = "PEM Certificate Signing Request"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = PemPrivateKey
    extension = "key"
    title = "PEM Private Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    extension = "pgp"
    title = "PGP Key/Signature"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = PemPublicKey
    extension = "pub"
    title = "PEM Public Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    extension = "p12"
    title = "Private Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = []

    format = Atom
    extension = "atom"
    title = "Atom"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = MsDosBatch
    extension = "bat"
    title = "Batch"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "c"
    title = "C"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cfc"
    title = "Coldfusion"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cfm"
    title = "Coldfusion"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = JavaClass
    extension = "class"
    title = "Java Class"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ClojureScript
    extension = "clj"
    title = "Clojure Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cmake"
    title = "cmake"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cmd"
    title = "Command"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "conf"
    title = "Conf"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cpp"
    title = "C++ Source"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cr"
    title = "Crystal"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "cs"
    title = "C#"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "css"
    title = "CSS"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "d"
    title = "D"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "dart"
    title = "Dart"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "dbml"
    title = "Database Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "dockerfile"
    title = "Dockerfile"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "dtd"
    title = "Document Type Definition"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ExecutableAndLinkableFormat
    extension = "elf"
    title = "Executable and Linkable Format"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ecor"
    title = "Eclipse Modeling Format"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "erl"
    title = "Erlang"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "exs"
    title = "Exilir"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "f90"
    title = "Fortran"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "fs"
    title = "F#"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ftl"
    title = "Fluent"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "funscript"
    title = "Funscript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "g6"
    title = "Graph6"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "go"
    title = "Go"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "goff"
    title = "Goff Configuration"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "gradle"
    title = "Gradle"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "groovy"
    title = "Groovy"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "h"
    title = "C Header"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "hcl"
    title = "HCL"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "hoa"
    title = "Hanoi Omega Automata"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "hpp"
    title = "C++ Header"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "hs"
    title = "Haskell"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = HypertextMarkupLanguage
    extension = "html"
    title = "HTML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "in"
    title = "in"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "java"
    title = "Java"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "jl"
    title = "Julia"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "js"
    title = "JavaScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "jsx"
    title = "JavaScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "kdl"
    title = "Cuddly Document Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "kt"
    title = "Kotlin"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "le"
    title = "Linear Executable"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "less"
    title = "Less"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "lisp"
    title = "Lisp"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "lmf"
    title = "Lexical Markup Framework"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "lock"
    title = "Lock"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = LuaScript
    extension = "lua"
    title = "Lua Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ml"
    title = "OCaml"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = MathematicalMarkupLanguage
    extension = "mathml"
    title = "MathML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "mustache"
    title = "Mustache"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "nim"
    title = "Nim"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "pas"
    title = "Pascal"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "patch"
    title = "Patch"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "pddl"
    title = "Planning Domain Definition"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "pe"
    title = "Portable Executable"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "php"
    title = "PHP"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = PerlScript
    extension = "pl"
    title = "Perl"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "pml"
    title = "Philipp's Modern Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "pnm"
    title = "Portable Any Map"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pnml"
    title = "Petri Net Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "properties"
    title = "Properties"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "proto"
    title = "ProtoBuffer"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ps1"
    title = "Powershell"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = PythonScript
    extension = "py"
    title = "Python"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "qasm"
    title = "Quantum Assembly Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "r"
    title = "R"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = RubyScript
    extension = "rb"
    title = "Ruby"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "rkt"
    title = "Racket"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "rpsl"
    title = "Routing Policy Specification Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "rs"
    title = "Rust"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ru"
    title = "Ruby"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "sbt"
    title = "Scala Build Tool"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "scala"
    title = "Scala"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "sdl"
    title = "Scenario Defined Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "sgml"
    title = "Standard Generalized Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ShellScript
    extension = "sh"
    title = "Shell Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "sql"
    title = "SQL"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "sum"
    title = "sum"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "swift"
    title = "Swift"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "tap"
    title = "Test Anything Protocol"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ToolCommandLanguageScript
    extension = "tcl"
    title = "Tool Command Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "tf"
    title = "Terraform"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "tfvar"
    title = "Terraform Variables"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ts"
    title = "TypeScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "thrift"
    title = "Thrift"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "toml"
    title = "TOML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "udl"
    title = "Universal Data Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "ur"
    title = "Ur"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "v"
    title = "V"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "vala"
    title = "Vala"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "vb"
    title = "Visual Basic"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "wit"
    title = "Wasm Interface Type"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "xcodeproj"
    title = "XCode Project"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ExtensibleMarkupLanguage
    extension = "xml"
    title = "Extensible Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = ExtensibleStylesheetLanguageTransformations
    extension = "xslt"
    title = "Extensible Stylesheet Language Transformation"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "yaml"
    title = "YAML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "yul"
    title = "Yul"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    extension = "yml"
    title = "YAML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = []

    format = AdobeIntegratedRuntime
    extension = "air"
    title = "Adobe Integrated Runtime"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsAppPackage
    extension = "appx"
    title = "Windows App Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AndroidPackage
    extension = "apk"
    title = "Android Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AndroidCompiledResources
    extension = "arsc"
    title = "Android Compiled Resources"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LlvmBitcode
    extension = "bc"
    title = "LLVM Bitcode"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "btf"
    title = "BPF Type Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CompoundFileBinary
    extension = "cfb"
    title = "Compound File Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CommonObjectFileFormat
    extension = "coff"
    title = "Common Object File Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GoogleChromeExtension
    extension = "crx"
    title = "Google Chrome Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DalvikExecutable
    extension = "dex"
    title = "Dalvik Executable"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OptimizedDalvikExecutable
    extension = "dey"
    title = "Dalvik Executable"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DynamicLinkLibrary
    extension = "dll"
    title = "Dynamic Link Library"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DebianBinaryPackage
    extension = "deb"
    title = "Debian Binary Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = EnterpriseApplicationArchive
    extension = "ear"
    title = "Java Enterprise Application Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ggjt"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ggmf"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ggml"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gguf"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = IosAppStorePackage
    extension = "ipa"
    title = "iOS App Store Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = JavaArchive
    extension = "jar"
    title = "Java Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ktest"
    title = "KTest Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = LuaBytecode
    extension = "luac"
    title = "Lua Bytecode"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MachO
    extension = "mach"
    title = "Mach-O"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mdx"
    title = "Mdict Dictionary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftSoftwareInstaller
    extension = "msi"
    title = "Microsoft Software Installer"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pbf"
    title = "ProtocolBuffer Binary Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = RedHatPackageManager
    extension = "rpm"
    title = "Red Hat Package Manager"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftVisualStudioSolution
    extension = "sln"
    title = "Microsoft Visual Studio Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Tasty
    extension = "tasty"
    title = "TASTy"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftVisualStudioExtension
    extension = "vsix"
    title = "Microsoft Visual Studio Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WebApplicationArchive
    extension = "war"
    title = "Java Web Application Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WebassemblyBinary
    extension = "wasm"
    title = "WebAssembly Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WebassemblyText
    extension = "wat"
    title = "WebAssembly Text"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Xap
    extension = "xap"
    title = "XAP"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Xpinstall
    extension = "xpi"
    title = "XPInstall"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "aba"
    title = "Australian Banking Association Records"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "abf"
    title = "Axon Binary Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftAccess2007Database
    extension = "accdb"
    title = "Microsoft Access 2007 Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ApacheArrowColumnar
    extension = "arrow"
    title = "Apache Arrow Columnar"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "arxml"
    title = "Autosar Arxml"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ApacheAvroObjectContainer
    extension = "avro"
    title = "Apache Avro Container"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bam"
    title = "Binary Alignment/Map"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bcf"
    title = "Binary Call Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bed"
    title = "Browser Extensible Data"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bgzf"
    title = "Blocked GZip Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bson"
    title = "Binary Javascript Object Notation"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cbor"
    title = "Concise Binary Object Representation"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cff"
    title = "Comtrade Data"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cnf"
    title = "DIMACS CNF"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "conllu"
    title = "CoNLL-U Annotations"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cram"
    title = "CRAM"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "csi"
    title = "Coordinate Sorted Index"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "csr"
    title = "Compressed Sparse Row"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cst"
    title = "Caret Seperated Text"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "csv"
    title = "Comma Seperated Values"
    category = Data
    indexer = Data_Csv
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "dbf"
    title = "dBase Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dif"
    title = "Data Interchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "edl"
    title = "Edit Decision List"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "fil"
    title = "SIGPROC Filter Bank"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gcode"
    title = "3D Printing Definition"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "gedcom"
    title = "Genealogical Family Tree"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gfa"
    title = "Graphical Fragment Assembly"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gff"
    title = "Generic Feature Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "graphviz"
    title = "Graphviz"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gtf"
    title = "Gene Transfer Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "hepmc2"
    title = "HepMC Collisions"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "hepmc3"
    title = "HepMC Collisions"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "i2"
    title = "MoTeC i2"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "igc"
    title = "IGC Flight Record"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = JsonFeed
    extension = "json"
    title = "JSON"
    category = Data
    indexer = Noop
    thumbnailer = Json
    opener = Noop
    viewer = Json
    cards = []

    extension = "kdbx"
    title = "Keepass Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ltx"
    title = "Lite Transaction File"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "m20"
    title = "Atomic Mass Evaluation 2020"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftAccessDatabase
    extension = "mdb"
    title = "Microsoft Access Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mdf"
    title = "Microsoft SQL Server Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mm1"
    title = "Metamath Zero Binary"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mtx"
    title = "Matrix Market"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mwa"
    title = "Murchison Widefield Array"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "obo"
    title = "OBO Ontology"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentDatabase
    extension = "odb"
    title = "OpenDocument Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "orc"
    title = "Optimized Row Columnar"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "owl"
    title = "OWL Ontology"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ApacheParquet
    extension = "parquet"
    title = "Parquet Columnar"
    category = Data
    indexer = Data_Parquet
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pcap"
    title = "Packet Capture"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pdb"
    title = "Protein Data Bank"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "prn"
    title = "Lotus Formatted Text"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "qif"
    title = "Quicken Interchange Format"
    category = Data
    indexer = Data_Qif
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "safetensors"
    title = "Safe Tensors"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sam"
    title = "Sequence Alignment/Map"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sarif"
    title = "Static Analysis Results Interchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "saz"
    title = "Fiddler Traffic"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "spdx"
    title = "Software Package Data Exchange"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sp3"
    title = "SP3 Precise GNSS Orbit"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Sqlite3
    extension = "sqlite"
    title = "SQLite3 Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "svf"
    title = "Serial Vector Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tbi"
    title = "Tabix"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tdms"
    title = "LabVIEW TDMS"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tsfile"
    title = "IoTDB Time Series"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tsp"
    title = "Travelling Salesman Problem Dataset"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ttl"
    title = "Turtle"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tzif"
    title = "Timezone Information Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "uxf"
    title = "Uniform eXchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "vbo"
    title = "VBOX Automotive Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "winmd"
    title = "Windows Metadata"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "xdr"
    title = "Stellar Open XDR"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "adf"
    title = "Amiga Floppy Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "b5t"
    title = "BlindWrite 5 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "b6t"
    title = "BlindWrite 6 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bwt"
    title = "BlindWrite 4 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "c2d"
    title = "Roxio-WinOnCD Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cdi"
    title = "DiscJuggler Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cif"
    title = "Easy CD Creator Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cue"
    title = "CDRWrite CUE Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "daa"
    title = "PowerISO Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleDiskImage
    extension = "dmg"
    title = "Apple Disk Image"
    category = DiskImage
    indexer = DiskImage_Dmg
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dms"
    title = "Amiga Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dsk"
    title = "ZX Spectrum Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Iso9660
    extension = "iso"
    title = "ISO 9660"
    category = DiskImage
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "nrg"
    title = "Nero Archive"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = QemuCopyOnWrite
    extension = "qcow"
    title = "QEMU Copy On Write"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sdi"
    title = "Virtual Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftVirtualHardDisk
    extension = "vhd"
    title = "Microsoft Virtual Hard Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftVirtualHardDisk2
    extension = "vhdx"
    title = "Microsoft Virtual Hard Disk 2"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = VirtualMachineDisk
    extension = "vmdk"
    title = "Virtual Machine Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = VirtualboxVirtualDiskImage
    extension = "vdi"
    title = "VirtualBox Virtual Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "123"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = Abiword
    extension = "abw"
    title = "AbiWord"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AbiwordTemplate
    extension = "awt"
    title = "AbiWord Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bib"
    title = "BibTex"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CircuitDiagramDocument
    extension = "cddx"
    title = "Circuit Diagram Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cook"
    title = "Cook Recipe"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftWordDocument
    extension = "doc"
    title = "Microsoft Word Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Docx
    cards = []

    format = OfficeOpenXmlDocument
    extension = "docx"
    title = "Microsoft Word Document"
    category = Document
    indexer = Document_Docx
    thumbnailer = Noop
    opener = Noop
    viewer = Docx
    cards = []

    extension = "dj"
    title = "Djot"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Drawio
    extension = "drawio"
    title = "Draw.io"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "123"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "fods"
    title = "Flat XML ODF Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "graffle"
    title = "OmniGraffle"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gslides"
    title = "Google Drive Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = IndesignMarkupLanguage
    extension = "idml"
    title = "Adobe InDesign Markup Language"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeIndesignDocument
    extension = "indd"
    title = "Adobe InDesign Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ipynb"
    title = "Interactive Python Notebook"
    category = Document
    indexer = Document_Ipynb
    thumbnailer = Ipynb
    opener = Noop
    viewer = Ipynb
    cards = []

    extension = "keynote"
    title = "Apple Keynote"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = Noop
    cards = []

    extension = "md"
    title = "Markdown"
    category = Document
    indexer = Document_Md
    thumbnailer = Md
    opener = Noop
    viewer = Markdown
    cards = []

    format = MicrosoftProjectPlan
    extension = "mpp"
    title = "Microsoft Project Plan"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "nb"
    title = "Mathematica Slideshow"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "nbp"
    title = "Mathematica Player Slideshow"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "numbers"
    title = "Apple Numbers"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = SheetJS
    cards = []

    format = OpendocumentFormula
    extension = "odf"
    title = "OpenDocument Formula"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentGraphics
    extension = "odg"
    title = "OpenDocument Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentTextMaster
    extension = "odm"
    title = "OpenDocument Text Master"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentPresentation
    extension = "odp"
    title = "OpenDocument Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentSpreadsheet
    extension = "ods"
    title = "OpenDocument Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = OpendocumentText
    extension = "odt"
    title = "OpenDocument Text"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentGraphicsTemplate
    extension = "otg"
    title = "OpenDocument Graphics Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentTextMasterTemplate
    extension = "otm"
    title = "OpenDocument Text Master Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OpendocumentSpreadsheetTemplate
    extension = "ots"
    title = "OpenDocument Spreadsheet Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "p7m"
    title = "CieSign Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pages"
    title = "Apple Pages"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pbix"
    title = "Microsoft PowerBI Report"
    category = Document
    indexer = Document_Pbix
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableDocumentFormat
    extension = "pdf"
    title = "PDF"
    category = Document
    indexer = Document_Pdf
    thumbnailer = Mac
    opener = Noop
    viewer = Pdf
    cards = []

    extension = "pez"
    title = "Prezi Desktop Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftPowerpointPresentation
    extension = "ppt"
    title = "Microsoft Powerpoint Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OfficeOpenXmlPresentation
    extension = "pptx"
    title = "Microsoft Powerpoint Presentation"
    category = Document
    indexer = Document_Pptx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "prz"
    title = "Lotus Freelance Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftPublisherDocument
    extension = "pub"
    title = "Microsoft Publisher Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "qpw"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = RichTextFormat
    extension = "rtf"
    title = "Rich Text Format"
    category = Document
    indexer = Document_Rtf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Stardraw
    extension = "sda"
    title = "StarDraw"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Starcalc
    extension = "sdc"
    title = "StarCalc"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Starimpress
    extension = "sdd"
    title = "StarImpress"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Starchart
    extension = "sds"
    title = "StarChart"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Starwriter
    extension = "sdw"
    title = "StarWriter"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlWriterGlobal
    extension = "sgw"
    title = "Sun XML Writer Global"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "shf"
    title = "ThinkFree Show"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Starmath
    extension = "smf"
    title = "StarMath"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlCalcTemplate
    extension = "stc"
    title = "Sun XML Calc Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlDrawTemplate
    extension = "std"
    title = "Sun XML Draw Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlWriterTemplate
    extension = "stw"
    title = "Sun XML Writer Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlCalc
    extension = "sxc"
    title = "Sun XML Calc"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlDraw
    extension = "sxd"
    title = "Sun XML Draw"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlImpress
    extension = "sxi"
    title = "Sun XML Impress"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlImpressTemplate
    extension = "sti"
    title = "Sun XML Impress Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlMath
    extension = "sxm"
    title = "Sun XML Math"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SunXmlWriter
    extension = "sxw"
    title = "Sun XML Writer"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "wb1"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wb2"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wb3"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wks"
    title = "Works 1.x-5.x Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wk1"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wk2"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wk3"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wk4"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wq1"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "wq2"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "xlr"
    title = "Works 6.x-9.x Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = WordperfectDocument
    extension = "wpd"
    title = "WordPerfect Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WordperfectGraphics
    extension = "wpg"
    title = "WordPerfect Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WordperfectMacro
    extension = "wpm"
    title = "WordPerfect Macro"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Latex
    extension = "tex"
    title = "LaTeX"
    category = Document
    indexer = Noop
    thumbnailer = Tex
    opener = Noop
    viewer = Latex
    cards = []

    format = PlainText
    extension = "txt"
    title = "Text"
    category = Document
    indexer = Document_Txt
    thumbnailer = Txt
    opener = Noop
    viewer = Text
    cards = []

    extension = "uos1"
    title = "Uniform Office Format Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "uos2"
    title = "Uniform Office Format Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = MicrosoftVisioDrawing
    extension = "vsd"
    title = "Microsoft Visio Drawing"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OfficeOpenXmlDrawing
    extension = "vsdx"
    title = "Microsoft Visio Drawing"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "webarchive"
    title = "Safari Web Archive"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftExcelSpreadsheet
    extension = "xls"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "xlsb"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    format = OfficeOpenXmlSpreadsheet
    extension = "xlsx"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Document_Xlsx
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = []

    extension = "emlx"
    title = "Apple Email Message"
    category = Email
    indexer = Email_Emlx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "eml"
    title = "Email Message"
    category = Email
    indexer = Email_Eml
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "msg"
    title = "Email Message"
    category = Email
    indexer = Email_Msg
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bdf"
    title = "Glyph Bitmap Distribution Format"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = EmbeddedOpentype
    extension = "eot"
    title = "Embedded OpenType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = BitmapFontBinary
    extension = "fnt"
    title = "Bitmap Font"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = Opentype
    extension = "otf"
    title = "OpenType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = Truetype
    extension = "ttf"
    title = "TrueType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = WebOpenFontFormat
    extension = "woff"
    title = "Web Open Font Format"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = WebOpenFontFormat2
    extension = "woff2"
    title = "Web Open Font Format 2"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = []

    format = Atari7800Rom
    extension = "a78"
    title = "Atari 7800 ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "acmi"
    title = "TacView Flight Plan"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bms"
    title = "Beat Saber Open Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bsor"
    title = "Beat Saber Open Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "bsp"
    title = "Valve Map"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "craft"
    title = "Kerbal Ship"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "d2s"
    title = "Diablo II Save"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dmx"
    title = "Valve Data Model eXchange"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "eqg"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "fgfp"
    title = "Flight Gear Flight Plan"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GameBoyAdvanceRom
    extension = "gba"
    title = "Game Boy Advance ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GameBoyColorRom
    extension = "gbc"
    title = "Game Boy Color ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GameBoyRom
    extension = "gb"
    title = "Game Boy ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gbx"
    title = "TrackMania Nations Game"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GameGearRom
    extension = "gg"
    title = "Game Gear ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gr2"
    title = "Granny2"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "hltas"
    title = "Half-Life TAS Script"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "jou"
    title = "Othello Players"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "manifest"
    title = "Riot Manifest"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mml"
    title = "Famicon Flavored Music Macro"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "msd"
    title = "StepMania"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mpq"
    title = "Blizzard Data File"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "nav"
    title = "SourceEngine Map"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NeoGeoPocketRom
    extension = "ngp"
    title = "Neo Geo Pocket ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NintendoDsRom
    extension = "nds"
    title = "Nintendo DS ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NintendoEntertainmentSystemRom
    extension = "nes"
    title = "Nintendo Entertainment System ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NintendoSwitchExecutable
    extension = "nso"
    title = "Nintendo Switch Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NintendoSwitchPackage
    extension = "nsp"
    title = "Nintendo Switch Package"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "os"
    title = "osu! Beatmap"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pak"
    title = "Unreal Pak"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pfs"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pgn"
    title = "Chess Portable Game Notation"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "rofl"
    title = "League of Legends Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "s3d"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sm"
    title = "StepMania"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sc2replay"
    title = "Starcraft 2 Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sqm"
    title = "Arma Mission"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SegaMasterSystemRom
    extension = "sms"
    title = "Sega Master System ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "trn"
    title = "Othello Tournaments"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "ttyrec"
    title = "TTY Recording"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "vdf"
    title = "Valve Data Format"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "vmf"
    title = "Valve Map Format"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "wld"
    title = "Everquest World"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Xbox360Executable
    extension = "xex"
    title = "Xbox 360 Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = XboxExecutable
    extension = "xbe"
    title = "Xbox Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NintendoSwitchRom
    extension = "xci"
    title = "Nintendo Switch ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Nintendo64Rom
    extension = "z64"
    title = "Nintendo 64 ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Autodesk123d
    extension = "123dx"
    title = "Autodesk 123D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Opennurbs
    extension = "3dm"
    title = "openNURBS"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = ThreeDimensionalStudio
    extension = "3ds"
    title = "3D Studio"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ThreeDimensionalManufacturingFormat
    extension = "3mf"
    title = "3D Manufacturing Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = Model3dAscii
    extension = "a3d"
    title = "Model 3D ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdditiveManufacturingFormat
    extension = "amf"
    title = "Additive Manufacturing Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "ase"
    title = "Aseprite Animation"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "b3d"
    title = "Blitz3d Game Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Blender
    extension = "blend"
    title = "Blender"
    category = Graphics
    indexer = Graphics_Blend
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "c3d"
    title = "C3D Motion Capture"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Cinema4d
    extension = "c4d"
    title = "Cinema 4D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DigitalAssetExchange
    extension = "dae"
    title = "Digital Asset Exchange"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GoogleDraco
    extension = "draco"
    title = "Google Draco"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = DesignWebFormat
    extension = "dwf"
    title = "Design Web Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DesignWebFormatXps
    extension = "dwfx"
    title = "Design Web Format XPS"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AutocadDrawing
    extension = "dwg"
    title = "AutoCAD Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DrawingExchangeFormatBinary
    extension = "dxf"
    title = "Drawing Exchange Format"
    category = Graphics
    indexer = Graphics_Dxf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "e57"
    title = "Lidar Point Cloud Data"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Fusion360
    extension = "f3d"
    title = "Fusion 360"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Filmbox
    extension = "fbx"
    title = "Filmbox"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "gldf"
    title = "Global Lighting Data Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "glif"
    title = "Glyph Interchange Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GlTransmissionFormatBinary
    extension = "glb"
    title = "GL Transmission Format Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "gltf"
    title = "GL Transmission Format Text"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "gvox"
    title = "General Voxel Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskInventorAssembly
    extension = "iam"
    title = "Autodesk Inventor Assembly"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskInventorDrawing
    extension = "idw"
    title = "Autodesk Inventor Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = InitialGraphicsExchangeSpecification
    extension = "iges"
    title = "Initial Graphics Exchange Specification"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskInventorPresentation
    extension = "ipn"
    title = "Autodesk Inventor Presentation"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskInventorPart
    extension = "ipt"
    title = "Autodesk Inventor Part"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = InterQuakeExport
    extension = "iqe"
    title = "Inter-Quake Export"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = InterQuakeModel
    extension = "iqm"
    title = "Inter-Quake Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = KhronosTexture
    extension = "ktx"
    title = "Khronos Texture"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = KhronosTexture2
    extension = "ktx2"
    title = "Khronos Texture 2"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "lw02"
    title = "Lightwave 3D Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = Model3dBinary
    extension = "m3d"
    title = "Model 3D Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MayaAscii
    extension = "ma"
    title = "Maya ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ThreeDimensionalStudioMax
    extension = "max"
    title = "3D Studio Max"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MayaBinary
    extension = "mb"
    title = "Maya Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mmd"
    title = "MikuMikuDance Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "msh"
    title = "Gmsh Mesh"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mtl"
    title = "Wavefront Material Library"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "obj"
    title = "Wavefront Object"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "off"
    title = "Mesh Object File Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PolygonBinary
    extension = "ply"
    title = "Polygon"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    extension = "pxr"
    title = "Pixar Image Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "rmesh"
    title = "Room Mesh"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SolidworksAssembly
    extension = "sldasm"
    title = "SolidWorks Assembly"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SolidworksDrawing
    extension = "slddrw"
    title = "SolidWorks Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SolidworksPart
    extension = "sldprt"
    title = "SolidWorks Part"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = SpaceclaimDocument
    extension = "scdoc"
    title = "SpaceClaim Document"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "sdf"
    title = "Simulation Description Format"
    category = Graphics
    indexer = Hardware_Sdf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = StandardForTheExchangeOfProductModelData
    extension = "step"
    title = "Standard for the Exchange of Products"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Sketchup
    extension = "skp"
    title = "SketchUp"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = StereolithographyAscii
    extension = "stl"
    title = "Stereolithography ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = Universal3d
    extension = "u3d"
    title = "Universal 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UniversalSceneDescriptionAscii
    extension = "usda"
    title = "Universal Scene Description ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UniversalSceneDescriptionBinary
    extension = "usdc"
    title = "Universal Scene Description Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = UniversalSceneDescriptionZipped
    extension = "usdz"
    title = "Universal Scene Description Zipped"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = VirtualRealityModelingLanguage
    extension = "vrml"
    title = "Virtual Reality Modeling Language"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = Magicavoxel
    extension = "vox"
    title = "MagicaVoxel"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = AutodeskAlias
    extension = "wire"
    title = "Autodesk Alias"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Extensible3d
    extension = "x3d"
    title = "Extensible 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Extensible3d
    extension = "x3db"
    title = "Extensible 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cir"
    title = "Spice Hardware Description"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "fld"
    title = "Ansys 3D Fields"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gdb"
    title = "Garmin GPS Database"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gds"
    title = "GDSII File Format"
    category = Hardware
    indexer = Hardware_Gds
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "lef"
    title = "Library Exchange Format"
    category = Hardware
    indexer = Hardware_Lef
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "veryl"
    title = "Veryl Hardware Description"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "3fr"
    title = "Hasselblad Raw"
    category = Image
    indexer = Noop
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    extension = "aai"
    title = "AAI Dune"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "art"
    title = "PFS: 1st Publisher"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "arw"
    title = "Sony Alpha Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = AdaptableScalableTextureCompression
    extension = "astc"
    title = "Adaptable Scalable Texture Compression"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeIllustratorArtwork
    extension = "ai"
    title = "Adobe Illustrator Artwork"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AnimatedPortableNetworkGraphics
    extension = "apng"
    title = "Animated Portable Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsAnimatedCursor
    extension = "avaniif"
    title = "Windows Animated Cursor"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Av1ImageFileFormat
    extension = "avif"
    title = "AV1 Image File Format"
    category = Image
    indexer = Noop
    thumbnailer = Avif
    opener = Noop
    viewer = Noop
    cards = []

    format = Av1ImageFileFormatSequence
    extension = "avifs"
    title = "AV1 Image File Format Sequence"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "avs"
    title = "AVS X"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsBitmap
    extension = "bmp"
    title = "Windows Bitmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Bmp
    opener = Noop
    viewer = Noop
    cards = []

    format = BetterPortableGraphics
    extension = "bpg"
    title = "Better Portable Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "brf"
    title = "Braille Ready Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "cals"
    title = "Continuous Acquisition and Life-cycle Support"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Cineon
    extension = "cin"
    title = "Cineon"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = CanonRaw
    extension = "crw"
    title = "Canon Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = CanonRaw2
    extension = "cr2"
    title = "Canon Raw 2"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = CanonRaw3
    extension = "cr3"
    title = "Canon Raw 3"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsCursor
    extension = "cur"
    title = "Windows Cursor"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DigitalImagingAndCommunicationsInMedicine
    extension = "dcm"
    title = "Digital Imaging and Communications in Medicine "
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dcr"
    title = "Kodak Digital Camera Raw "
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftDirectdrawSurface
    extension = "dds"
    title = "Microsoft DirectDraw Surface"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Dds
    opener = Noop
    viewer = Three
    cards = []

    extension = "dng"
    title = "Adobe DNG Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dot"
    title = "Graph Visualization"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "dpx"
    title = "SMPTE Digital Moving Picture Exchange"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Djvu
    extension = "dvju"
    title = "DjVu"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = DigitalPictureExchange
    extension = "dpx"
    title = "Digital Picture Exchange"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "emf"
    title = "Microsoft Enhanced Metafile"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = EncapsulatedPostscript
    extension = "eps"
    title = "Encapsulated PostScript"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Openexr
    extension = "exr"
    title = "OpenEXR"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = []

    format = ExtensibleStylesheetLanguageTransformations
    extension = "ff"
    title = "Farbfeld"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = FreeLosslessImageFormat
    extension = "flif"
    title = "Free Lossless Image Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "fpx"
    title = "FlashPix"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GraphicsInterchangeFormat
    extension = "gif"
    title = "Graphics Interchange Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Gif
    opener = Noop
    viewer = Noop
    cards = []

    extension = "gpmf"
    title = "GoPro Metadata Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = RadianceHdr
    extension = "hdr"
    title = "Radiance HDR"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = HighEfficiencyImageCoding
    extension = "heic"
    title = "High Efficiency Image Coding"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Heif
    opener = Noop
    viewer = Noop
    cards = []

    format = HighEfficiencyImageCodingSequence
    extension = "heics"
    title = "High Efficiency Image Coding Sequence"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = HighEfficiencyImageFileFormat
    extension = "heif"
    title = "High Efficiency Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Heif
    opener = Noop
    viewer = Noop
    cards = []

    format = HighEfficiencyImageFileFormatSequence
    extension = "heifs"
    title = "High Efficiency Image File Format Sequence"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "hvif"
    title = "Haiku Vector Icon Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = IccProfile
    extension = "icc"
    title = "ICC Profile"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleIconImage
    extension = "icns"
    title = "Apple Icon Image"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsIcon
    extension = "ico"
    title = "Windows Icon"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Ico
    opener = Noop
    viewer = Noop
    cards = []

    format = Jpeg2000Codestream
    extension = "j2c"
    title = "JPEG 2000 Codestream"
    category = Image
    indexer = Noop
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JpegLs
    extension = "jls"
    title = "JPEG-LS"
    category = Image
    indexer = Noop
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JpegNetworkGraphics
    extension = "jng"
    title = "JPEG Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = Jpeg2000Part1
    extension = "jp2"
    title = "JPEG 2000 Part 1"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JointPhotographicExpertsGroup
    extension = "jpeg"
    title = "JPEG"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JointPhotographicExpertsGroup
    extension = "jpg"
    title = "JPEG"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = Jpeg2000Part6
    extension = "jpm"
    title = "JPEG 2000 Part 6"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Jpeg2000Part2
    extension = "jpx"
    title = "JPEG 2000 Part 2"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JpegExtendedRange
    extension = "jxr"
    title = "JPEG Extended Range"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    format = JpegXl
    extension = "jxl"
    title = "JPEG-XL"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mat"
    title = "MATLAB Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MagickImageFileFormat
    extension = "miff"
    title = "Magick Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = MultipleImageNetworkGraphics
    extension = "mng"
    title = "Multiple-image Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mrw"
    title = "Sony (Minolta) Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mtv"
    title = "MTV Raytracing"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = NikonElectronicFile
    extension = "nef"
    title = "Nikon Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = Openraster
    extension = "ora"
    title = "OpenRaster"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = OlympusRawFormat
    extension = "orf"
    title = "Olympus Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableArbitraryMap
    extension = "pam"
    title = "Portable Arbitrary Map"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableBitmap
    extension = "pbm"
    title = "Portable Bitmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "pef"
    title = "Pentax Electronic File"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableFloatmap
    extension = "pfm"
    title = "Portable Floatmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableGraymap
    extension = "pgm"
    title = "Portable Graymap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = PortableNetworkGraphics
    extension = "png"
    title = "Portable Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Png
    opener = Noop
    viewer = Noop
    cards = []

    format = PortablePixmap
    extension = "ppm"
    title = "Portable PixMap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Ppm
    opener = Noop
    viewer = Noop
    cards = []

    format = Postscript
    extension = "ps"
    title = "PostScript"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobePhotoshopDocument
    extension = "psd"
    title = "Adobe Photoshop"
    category = Image
    indexer = Image_Psd
    thumbnailer = Psd
    opener = Noop
    viewer = Noop
    cards = []

    format = QuiteOkImage
    extension = "qoi"
    title = "Quick OK Image"
    category = Image
    indexer = Noop
    thumbnailer = Qoi
    opener = Noop
    viewer = Noop
    cards = []

    format = FujifilmRaw
    extension = "raf"
    title = "Fujifilm Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = PanasonicRaw
    extension = "rw2"
    title = "Panasonic Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    extension = "rwl"
    title = "Leica Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    format = SiliconGraphicsImage
    extension = "sgi"
    title = "Silicon Graphics Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ScalableVectorGraphics
    extension = "svg"
    title = "Scalable Vector Graphics"
    category = Image
    indexer = Image_Svg
    thumbnailer = Svg
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tga"
    title = "Truevision Targa Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tga
    opener = Noop
    viewer = Noop
    cards = []

    extension = "tif"
    title = "Tag Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tiff
    opener = Noop
    viewer = Noop
    cards = []

    format = TagImageFileFormat
    extension = "tiff"
    title = "TIFF"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tiff
    opener = Noop
    viewer = Noop
    cards = []

    format = ExperimentalComputingFacility
    extension = "xcf"
    title = "Experimental Computing Facility"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Webp
    extension = "webp"
    title = "WebP"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Webp
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsMetafile
    extension = "wmf"
    title = "Windows Metafile"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "wpg"
    title = "Word Perfect Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "x3f"
    title = "Sigma Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = []

    extension = "xisf"
    title = "Extensible Image Serialization Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = XPixmap
    extension = "xpm"
    title = "X PixMap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = FlexibleAndInteroperableDataTransfer
    extension = "fit"
    title = "Flexible and Interoperable Data Transfer"
    category = Map
    indexer = Map_Fit
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = FlexibleImageTransportSystem
    extension = "fits"
    title = "Flexible Image Transport System"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = GpsExchangeFormat
    extension = "gpx"
    title = "GPS Exchange Format"
    category = Map
    indexer = Map_Gpx
    thumbnailer = Noop
    opener = Noop
    viewer = Leaflet
    cards = []

    format = GeographyMarkupLanguage
    extension = "gml"
    title = "Geography Markup Language"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = KeyholeMarkupLanguage
    extension = "kml"
    title = "Keyhole Markup Language"
    category = Map
    indexer = Map_Kml
    thumbnailer = Noop
    opener = Noop
    viewer = Leaflet
    cards = []

    format = KeyholeMarkupLanguageZipped
    extension = "kmz"
    title = "Keyhole Markup Language Zipped"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "loc"
    title = "GPS Location"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "osrm"
    title = "OpenStreetMap Route"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "rinex"
    title = "RTCM Receiver Independent Exchange"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = Shapefile
    extension = "shp"
    title = "Shapefile"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = TiledTilesetXml
    extension = "tsx"
    title = "Tiled Tileset XML"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = TimedTextMarkupLanguage
    extension = "ttml"
    title = "Timed Text Markup Language"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = TrainingCenterXml
    extension = "tcx"
    title = "Training Center XML"
    category = Map
    indexer = Map_Tcx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    format = ThirdGenerationPartnershipProject
    extension = "3gp"
    title = "3rd Generation Partnership Project"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = ThirdGenerationPartnershipProject
    extension = "3gpp"
    title = "3rd Generation Partnership Project"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = ThirdGenerationPartnershipProject2
    extension = "3gpp2"
    title = "3rd Generation Partnership Project 2"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = ActionsMediaVideo
    extension = "amv"
    title = "Actions Media Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AudioVideoInterleave
    extension = "avi"
    title = "Audio Video Interleave"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = MicrosoftDigitalVideoRecording
    extension = "dvr-ms"
    title = "Microsoft Digital Video Recording"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeFlashPlayerProtectedVideo
    extension = "f4p"
    title = "Adobe Flash Player Protected Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AdobeFlashPlayerVideo
    extension = "f4v"
    title = "Adobe Flash Player Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskAnimatorPro
    extension = "flc"
    title = "Autodesk Animator Pro"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AutodeskAnimator
    extension = "fli"
    title = "Autodesk Animator"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = FlashVideo
    extension = "flv"
    title = "Adobe Flash Player Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = BdavMpeg2TransportStream
    extension = "m2ts"
    title = "MPEG-2 Transport Stream"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleItunesVideo
    extension = "m4v"
    title = "Apple iTunes Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Jpeg2000Part3
    extension = "mj2"
    title = "JPEG 2000 Part 3"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Matroska3dVideo
    extension = "mk3d"
    title = "Matroska 3D Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = MatroskaVideo
    extension = "mkv"
    title = "Matroska Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = AppleQuicktime
    extension = "mov"
    title = "Apple QuickTime Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Mpeg12Video
    extension = "mpg"
    title = "MPEG-1/2 Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    extension = "mpls"
    title = "Movie Playlist"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

   format = Mpeg4Part14Video
    extension = "mp4"
    title = "MPEG-4 Part 14 Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Mtv
    extension = "mtv"
    title = "MTV"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = SonyMovie
    extension = "mqv"
    title = "Sony Movie"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = MaterialExchangeFormat
    extension = "mxf"
    title = "Material Exchange Format"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = OggMedia
    extension = "ogm"
    title = "Ogg Media"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = OggTheora
    extension = "ogv"
    title = "Ogg Theora"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Realvideo
    extension = "rv"
    title = "RealVideo"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    extension = "srt"
    title = "SubRip Subtitles"
    category = Video
    indexer = Video_Srt
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = []

    extension = "vcxproj"
    title = "Visual Studio Project"
    category = Code
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Code
    cards = []

    extension = "vob"
    title = "VOB"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = Webm
    extension = "webm"
    title = "WebM"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsMediaVideo
    extension = "wmv"
    title = "Windows Media Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []

    format = WindowsRecordedTvShow
    extension = "wtv"
    title = "Windows Recorded TV Show"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = []
}