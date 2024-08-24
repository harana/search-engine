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
    cards = ["thumbnail", "file_info"]

    format = ArbitraryBinaryData
    extension = "bin"
    title = "Arbitrary Binary Data"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MsDosExecutable
    extension = "exe"
    title = "MS-DOS Executable"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LinearExecutable
    extension = "le"
    title = "Linear Executable"
    category = Application
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SevenZip
    extension = "7z"
    title = "7-Zip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UnixArchiver
    extension = "a"
    title = "UNIX archiver"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Ace
    extension = "ace"
    title = "Advanced Compression Engine"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Alz
    extension = "alz"
    title = "ALZip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ArchivedByRobertJung
    extension = "ar"
    title = "Archived by Robert Jung"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Bzip
    extension = "bz"
    title = "Bzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Bzip2
    extension = "bz2"
    title = "Bzip2"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Bzip3
    extension = "bz3"
    title = "Bzip2"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Cabinet
    extension = "cab"
    title = "Cabinet"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "compress"
    title = "Unix Compress"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Cpio
    extension = "cpio"
    title = "CPIO"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Gzip
    extension = "gz"
    title = "Gzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Lha
    extension = "lha"
    title = "LHA"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LempelZivFiniteStateEntropy
    extension = "lzfse"
    title = "Lempel-Ziv Finite State Entropy"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Lzip
    extension = "lz"
    title = "Lzip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Lzip
    extension = "lzh"
    title = "Lziph"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Lz4
    extension = "lz4"
    title = "LZ4"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LempelZivMarkovChainAlgorithm
    extension = "lzma"
    title = "Lempel-Ziv-Markov chain algorithm"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Lzop
    extension = "lzo"
    title = "Lzop"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Larc
    extension = "lzs"
    title = "LArc"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LongRangeZip
    extension = "lrzip"
    title = "Long Range Zip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mtree"
    title = "Mtree"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pax"
    title = "Pax"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Pmarc
    extension = "pma"
    title = "PMarc"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = RoshalArchive
    extension = "rar"
    title = "Roshal Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Rzip
    extension = "rz"
    title = "Rzip"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Seqbox
    extension = "sbx"
    title = "SeqBox"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "shar"
    title = "Shar"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Snappy
    extension = "sz"
    title = "Snappy"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Stuffit
    extension = "sit"
    title = "StuffIt"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = StuffitX
    extension = "sitx"
    title = "StuffIt X"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Squashfs
    extension = "squashfs"
    title = "Squashfs"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = TapeArchive
    extension = "tar"
    title = "Tape Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ExtensibleArchive
    extension = "xar"
    title = "Extensible Archive"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Xz
    extension = "xz"
    title = "Xz"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "warc"
    title = "Warc"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsImagingFormat
    extension = "wim"
    title = "Windows Imaging Format"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UnixCompress
    extension = "Z"
    title = "UNIX compress"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Zip
    extension = "zip"
    title = "Zip"
    category = Archive
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Zoo
    extension = "zoo"
    title = "Zoo"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Zpaq
    extension = "zpaq"
    title = "ZPAQ"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Zstandard
    extension = "zstd"
    title = "Zstandard"
    category = Archive
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = EightBitSampledVoice
    extension = "8svx"
    title = "8-Bit Sampled Voice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdvancedAudioCoding
    extension = "aac"
    title = "Advanced Audio Coding"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AudioCodec3
    extension = "ac3"
    title = "Audio Codec 3"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AudioInterchangeFileFormat
    extension = "aiff"
    title = "Audio Interchange File Format"
    category = Audio
    indexer = Noop
    thumbnailer = Aiff
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "alac"
    title = "Apple Lossless Audio Codec"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdaptiveMultiRate
    extension = "amr"
    title = "Adaptive Multi-Rate"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MonkeysAudio
    extension = "ape"
    title = "Monkey's Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "at3"
    title = "Sony UMD Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Au
    extension = "au"
    title = "Audio Unit"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AudioVisualResearch
    extension = "avr"
    title = "Audio Visual Research"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CdAudio
    extension = "cda"
    title = "CD Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SonyDsdStreamFile
    extension = "dsf"
    title = "Sony DSD Stream File"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlashMp4Audio
    extension = "f4a"
    title = "Adobe Flash Player Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlashMp4Audiobook
    extension = "f4b"
    title = "Adobe Flash Player Audiobook"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FreeLosslessAudioCodec
    extension = "flac"
    title = "Free Lossless Audio Codec"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ImpulseTrackerModule
    extension = "it"
    title = "Impulse Tracker Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "lrc"
    title = "Lyrics"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleItunesAudio
    extension = "m4a"
    title = "Apple Music Audio"
    category = Audio
    indexer = Noop
    thumbnailer = M4a
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleItunesAudiobook
    extension = "m4b"
    title = "Apple Music Audiobook"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleItunesProtectedAudio
    extension = "m4p"
    title = "Apple Music Protected Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MatroskaAudio
    extension = "mka"
    title = "Matroska Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MusicalInstrumentDigitalInterface
    extension = "midi"
    title = "Musical Instrument Digital Interface"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mmf"
    title = "Yamaha Synthetic Music"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UltimateSoundtrackerModule
    extension = "mod"
    title = "Ultimate Soundtracker Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mp1"
    title = "MPEG-1/2 Audio Layer 1"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Mpeg12AudioLayer2
    extension = "mp2"
    title = "MPEG-1/2 Audio Layer 2"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Mpeg12AudioLayer3
    extension = "mp3"
    title = "MPEG-1/2 Audio Layer 3"
    category = Audio
    indexer = Noop
    thumbnailer = Mp3
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Musepack
    extension = "mpc"
    title = "Musepack"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mus"
    title = "Music Time Internal"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Musicxml
    extension = "musicxml"
    title = "MusicXML"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MusicxmlZip
    extension = "mxl"
    title = "MusicXML Zip"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ncm"
    title = "Netease Cloud Music"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ncw"
    title = "Native Instruments Compressed Wave"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggFlac
    extension = "oga"
    title = "Ogg FLAC"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggVorbis
    extension = "ogg"
    title = "Ogg Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggMultiplexedMedia
    extension = "ogx"
    title = "Ogg Multiplexed Media"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggOpus
    extension = "opus"
    title = "Ogg Opus"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "psf"
    title = "Parameter Storage Format"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = QualcommPurevoice
    extension = "qcp"
    title = "Qualcomm PureVoice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = QuiteOkAudio
    extension = "qoa"
    title = "Quite OK Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Realaudio
    extension = "ra"
    title = "RealAudio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ScreamTracker3Module
    extension = "s3m"
    title = "Scream Tracker 3 Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Soundfont2
    extension = "sf2"
    title = "SoundFont 2"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sf3"
    title = "SoundFont 3"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sfz"
    title = "SoundFont Z"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggSpeex
    extension = "spx"
    title = "Ogg Speex"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Fasttracker2ExtendedModule
    extension = "xm"
    title = "FastTracker 2 Extended Module"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CreativeVoice
    extension = "voc"
    title = "Creative Voice"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WaveformAudio
    extension = "wav"
    title = "Waveform Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Wav
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Wavpack
    extension = "wv"
    title = "WavPack"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsMediaAudio
    extension = "wma"
    title = "Windows Media Audio"
    category = Audio
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "awz"
    title = "Amazon Kindle Book"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "awz3"
    title = "Amazon Kindle Book"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = BroadBandEbook
    extension = "bbeb"
    title = "Broad Band eBook"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ElectronicPublication
    extension = "epub"
    title = "Electronic Publication"
    category = Book
    indexer = Book_Epub
    thumbnailer = Noop
    opener = Noop
    viewer = Epub
    cards = ["thumbnail", "file_info"]

    format = Fictionbook
    extension = "fb2"
    title = "FictionBook"
    category = Book
    indexer = Book_Fb2
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FictionbookZip
    extension = "fbz"
    title = "FictionBook Zip"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftReader
    extension = "lit"
    title = "Microsoft Reader"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Mobipocket
    extension = "mobi"
    title = "Mobipocket"
    category = Book
    indexer = Book_Mobi
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ogam"
    title = "OGAM Markup Language"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "rm"
    title = "Remarkable"
    category = Book
    indexer = Book_Rm
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tw"
    title = "Twine Story"
    category = Book
    indexer = Book_Twee
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "twee"
    title = "Twine Story"
    category = Book
    indexer = Book_Twee
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "usfm"
    title = "Unified Standard Format Marker"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "zim"
    title = "Zim Wiki Extract"
    category = Book
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Icalendar
    extension = "ics"
    title = "iCalendar"
    category = Calendar
    indexer = Calendar_Ical
    thumbnailer = Noop
    opener = Noop
    viewer = Calendar
    cards = ["thumbnail", "file_info"]

    extension = "vcf"
    title = "Variant Call Format"
    category = Contact
    indexer = Contact_Vcard
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PgpMessage
    extension = "asc"
    title = "PGP Message"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PgpPrivateKeyBlock
    extension = "asc"
    title = "PGP Key Block"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PgpSignature
    extension = "asc"
    title = "PGP Signature"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = PgpSignedMessage
    extension = "asc"
    title = "PGP Signed Message"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = DerCertificate
    extension = "der"
    title = "DER Certificate"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JavaKeystore
    extension = "jks"
    title = "Java KeyStore"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = PemCertificate
    extension = "crt"
    title = "PEM Certificate"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = PemCertificateSigningRequest
    extension = "crt"
    title = "PEM Certificate Signing Request"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = PemPrivateKey
    extension = "key"
    title = "PEM Private Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    extension = "pgp"
    title = "PGP Key/Signature"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = PemPublicKey
    extension = "pub"
    title = "PEM Public Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    extension = "p12"
    title = "Private Key"
    category = Certificate
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Certificate
    cards = ["thumbnail", "file_info"]

    format = Atom
    extension = "atom"
    title = "Atom"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = MsDosBatch
    extension = "bat"
    title = "Batch"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "c"
    title = "C"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cfc"
    title = "Coldfusion"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cfm"
    title = "Coldfusion"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = JavaClass
    extension = "class"
    title = "Java Class"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ClojureScript
    extension = "clj"
    title = "Clojure Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cmake"
    title = "cmake"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cmd"
    title = "Command"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "conf"
    title = "Conf"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cpp"
    title = "C++ Source"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cr"
    title = "Crystal"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "cs"
    title = "C#"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "css"
    title = "CSS"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "d"
    title = "D"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "dart"
    title = "Dart"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "dbml"
    title = "Database Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "dockerfile"
    title = "Dockerfile"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "dtd"
    title = "Document Type Definition"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ExecutableAndLinkableFormat
    extension = "elf"
    title = "Executable and Linkable Format"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ecor"
    title = "Eclipse Modeling Format"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "erl"
    title = "Erlang"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "exs"
    title = "Exilir"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "f90"
    title = "Fortran"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "fs"
    title = "F#"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ftl"
    title = "Fluent"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "funscript"
    title = "Funscript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "g6"
    title = "Graph6"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "go"
    title = "Go"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "goff"
    title = "Goff Configuration"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "gradle"
    title = "Gradle"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "groovy"
    title = "Groovy"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "h"
    title = "C Header"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "hcl"
    title = "HCL"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "hoa"
    title = "Hanoi Omega Automata"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "hpp"
    title = "C++ Header"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "hs"
    title = "Haskell"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = HypertextMarkupLanguage
    extension = "html"
    title = "HTML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "in"
    title = "in"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "java"
    title = "Java"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "jl"
    title = "Julia"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "js"
    title = "JavaScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "jsx"
    title = "JavaScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "kdl"
    title = "Cuddly Document Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "kt"
    title = "Kotlin"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "le"
    title = "Linear Executable"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "less"
    title = "Less"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "lisp"
    title = "Lisp"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "lmf"
    title = "Lexical Markup Framework"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "lock"
    title = "Lock"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = LuaScript
    extension = "lua"
    title = "Lua Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ml"
    title = "OCaml"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = MathematicalMarkupLanguage
    extension = "mathml"
    title = "MathML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "mustache"
    title = "Mustache"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "nim"
    title = "Nim"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "pas"
    title = "Pascal"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "patch"
    title = "Patch"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "pddl"
    title = "Planning Domain Definition"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "pe"
    title = "Portable Executable"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "php"
    title = "PHP"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = PerlScript
    extension = "pl"
    title = "Perl"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "pml"
    title = "Philipp's Modern Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "pnm"
    title = "Portable Any Map"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pnml"
    title = "Petri Net Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "properties"
    title = "Properties"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "proto"
    title = "ProtoBuffer"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ps1"
    title = "Powershell"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = PythonScript
    extension = "py"
    title = "Python"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "qasm"
    title = "Quantum Assembly Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "r"
    title = "R"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = RubyScript
    extension = "rb"
    title = "Ruby"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "rkt"
    title = "Racket"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "rpsl"
    title = "Routing Policy Specification Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "rs"
    title = "Rust"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ru"
    title = "Ruby"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "sbt"
    title = "Scala Build Tool"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "scala"
    title = "Scala"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "sdl"
    title = "Scenario Defined Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "sgml"
    title = "Standard Generalized Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ShellScript
    extension = "sh"
    title = "Shell Script"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "sql"
    title = "SQL"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "sum"
    title = "sum"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "swift"
    title = "Swift"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "tap"
    title = "Test Anything Protocol"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ToolCommandLanguageScript
    extension = "tcl"
    title = "Tool Command Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "tf"
    title = "Terraform"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "tfvar"
    title = "Terraform Variables"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ts"
    title = "TypeScript"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "thrift"
    title = "Thrift"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "toml"
    title = "TOML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "udl"
    title = "Universal Data Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "ur"
    title = "Ur"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "v"
    title = "V"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "vala"
    title = "Vala"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "vb"
    title = "Visual Basic"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "wit"
    title = "Wasm Interface Type"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "xcodeproj"
    title = "XCode Project"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ExtensibleMarkupLanguage
    extension = "xml"
    title = "Extensible Markup Language"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = ExtensibleStylesheetLanguageTransformations
    extension = "xslt"
    title = "Extensible Stylesheet Language Transformation"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "yaml"
    title = "YAML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "yul"
    title = "Yul"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "yml"
    title = "YAML"
    category = Code
    indexer = Code_Comments
    thumbnailer = Code
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    format = AdobeIntegratedRuntime
    extension = "air"
    title = "Adobe Integrated Runtime"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsAppPackage
    extension = "appx"
    title = "Windows App Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AndroidPackage
    extension = "apk"
    title = "Android Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AndroidResourceStorageContainer
    extension = "arsc"
    title = "Android Compiled Resources"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LlvmBitcode
    extension = "bc"
    title = "LLVM Bitcode"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "btf"
    title = "BPF Type Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CompoundFileBinary
    extension = "cfb"
    title = "Compound File Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CommonObjectFileFormat
    extension = "coff"
    title = "Common Object File Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GoogleChromeExtension
    extension = "crx"
    title = "Google Chrome Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DalvikExecutable
    extension = "dex"
    title = "Dalvik Executable"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OptimizedDalvikExecutable
    extension = "dey"
    title = "Dalvik Executable"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DynamicLinkLibrary
    extension = "dll"
    title = "Dynamic Link Library"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DebianPackage
    extension = "deb"
    title = "Debian Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = EnterpriseApplicationArchive
    extension = "ear"
    title = "Java Enterprise Application Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ggjt"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ggmf"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ggml"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gguf"
    title = "Machine Learning Model"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = IosAppStorePackage
    extension = "ipa"
    title = "iOS App Store Package"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JavaArchive
    extension = "jar"
    title = "Java Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ktest"
    title = "KTest Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = LuaBytecode
    extension = "luac"
    title = "Lua Bytecode"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MachO
    extension = "mach"
    title = "Mach-O"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mdx"
    title = "Mdict Dictionary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftSoftwareInstaller
    extension = "msi"
    title = "Microsoft Software Installer"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pbf"
    title = "ProtocolBuffer Binary Format"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = RedHatPackageManager
    extension = "rpm"
    title = "Red Hat Package Manager"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftVisualStudioSolution
    extension = "sln"
    title = "Microsoft Visual Studio Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Tasty
    extension = "tasty"
    title = "TASTy"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftVisualStudioExtension
    extension = "vsix"
    title = "Microsoft Visual Studio Extension"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WebApplicationArchive
    extension = "war"
    title = "Java Web Application Archive"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WebassemblyBinary
    extension = "wasm"
    title = "WebAssembly Binary"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WebassemblyText
    extension = "wat"
    title = "WebAssembly Text"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Xap
    extension = "xap"
    title = "XAP"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Xpinstall
    extension = "xpi"
    title = "XPInstall"
    category = CodeArtifact
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "aba"
    title = "Australian Banking Association Records"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "abf"
    title = "Axon Binary Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftAccess2007Database
    extension = "accdb"
    title = "Microsoft Access 2007 Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ApacheArrowColumnar
    extension = "arrow"
    title = "Apache Arrow Columnar"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "arxml"
    title = "Autosar Arxml"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ApacheAvro
    extension = "avro"
    title = "Apache Avro Container"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bam"
    title = "Binary Alignment/Map"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bcf"
    title = "Binary Call Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bed"
    title = "Browser Extensible Data"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bgzf"
    title = "Blocked GZip Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bson"
    title = "Binary Javascript Object Notation"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cbor"
    title = "Concise Binary Object Representation"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cff"
    title = "Comtrade Data"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cnf"
    title = "DIMACS CNF"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "conllu"
    title = "CoNLL-U Annotations"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cram"
    title = "CRAM"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "csi"
    title = "Coordinate Sorted Index"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "csr"
    title = "Compressed Sparse Row"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cst"
    title = "Caret Seperated Text"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "csv"
    title = "Comma Seperated Values"
    category = Data
    indexer = Data_Csv
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "dbf"
    title = "dBase Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dif"
    title = "Data Interchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "edl"
    title = "Edit Decision List"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "fil"
    title = "SIGPROC Filter Bank"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gcode"
    title = "3D Printing Definition"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "gedcom"
    title = "Genealogical Family Tree"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gfa"
    title = "Graphical Fragment Assembly"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gff"
    title = "Generic Feature Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "graphviz"
    title = "Graphviz"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gtf"
    title = "Gene Transfer Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "hepmc2"
    title = "HepMC Collisions"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "hepmc3"
    title = "HepMC Collisions"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "i2"
    title = "MoTeC i2"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "igc"
    title = "IGC Flight Record"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JsonFeed
    extension = "json"
    title = "JSON"
    category = Data
    indexer = Noop
    thumbnailer = Json
    opener = Noop
    viewer = Json
    cards = ["thumbnail", "file_info"]

    extension = "kdbx"
    title = "Keepass Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ltx"
    title = "Lite Transaction File"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "m20"
    title = "Atomic Mass Evaluation 2020"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftAccessDatabase
    extension = "mdb"
    title = "Microsoft Access Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mdf"
    title = "Microsoft SQL Server Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mm1"
    title = "Metamath Zero Binary"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mtx"
    title = "Matrix Market"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mwa"
    title = "Murchison Widefield Array"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "obo"
    title = "OBO Ontology"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentDatabase
    extension = "odb"
    title = "OpenDocument Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "orc"
    title = "Optimized Row Columnar"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "owl"
    title = "OWL Ontology"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ApacheParquet
    extension = "parquet"
    title = "Parquet Columnar"
    category = Data
    indexer = Data_Parquet
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pcap"
    title = "Packet Capture"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pdb"
    title = "Protein Data Bank"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "prn"
    title = "Lotus Formatted Text"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "qif"
    title = "Quicken Interchange Format"
    category = Data
    indexer = Data_Qif
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "safetensors"
    title = "Safe Tensors"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sam"
    title = "Sequence Alignment/Map"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sarif"
    title = "Static Analysis Results Interchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "saz"
    title = "Fiddler Traffic"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "spdx"
    title = "Software Package Data Exchange"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sp3"
    title = "SP3 Precise GNSS Orbit"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Sqlite3
    extension = "sqlite"
    title = "SQLite3 Database"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "svf"
    title = "Serial Vector Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tbi"
    title = "Tabix"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tdms"
    title = "LabVIEW TDMS"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tsfile"
    title = "IoTDB Time Series"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tsp"
    title = "Travelling Salesman Problem Dataset"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ttl"
    title = "Turtle"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tzif"
    title = "Timezone Information Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "uxf"
    title = "Uniform eXchange Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "vbo"
    title = "VBOX Automotive Format"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "winmd"
    title = "Windows Metadata"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "xdr"
    title = "Stellar Open XDR"
    category = Data
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "adf"
    title = "Amiga Floppy Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "b5t"
    title = "BlindWrite 5 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "b6t"
    title = "BlindWrite 6 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bwt"
    title = "BlindWrite 4 Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "c2d"
    title = "Roxio-WinOnCD Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cdi"
    title = "DiscJuggler Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cif"
    title = "Easy CD Creator Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cue"
    title = "CDRWrite CUE Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "daa"
    title = "PowerISO Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleDiskImage
    extension = "dmg"
    title = "Apple Disk Image"
    category = DiskImage
    indexer = DiskImage_Dmg
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dms"
    title = "Amiga Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dsk"
    title = "ZX Spectrum Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Iso9660
    extension = "iso"
    title = "ISO 9660"
    category = DiskImage
    indexer = Archive
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "nrg"
    title = "Nero Archive"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = QemuCopyOnWrite
    extension = "qcow"
    title = "QEMU Copy On Write"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sdi"
    title = "Virtual Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftVirtualHardDisk
    extension = "vhd"
    title = "Microsoft Virtual Hard Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftVirtualHardDisk2
    extension = "vhdx"
    title = "Microsoft Virtual Hard Disk 2"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = VirtualMachineDisk
    extension = "vmdk"
    title = "Virtual Machine Disk"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = VirtualboxVirtualDiskImage
    extension = "vdi"
    title = "VirtualBox Virtual Disk Image"
    category = DiskImage
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "123"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = Abiword
    extension = "abw"
    title = "AbiWord"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AbiwordTemplate
    extension = "awt"
    title = "AbiWord Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bib"
    title = "BibTex"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CircuitDiagramDocument
    extension = "cddx"
    title = "Circuit Diagram Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cook"
    title = "Cook Recipe"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftWordDocument
    extension = "doc"
    title = "Microsoft Word Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Docx
    cards = ["thumbnail", "file_info"]

    format = OfficeOpenXmlDocument
    extension = "docx"
    title = "Microsoft Word Document"
    category = Document
    indexer = Document_Docx
    thumbnailer = Noop
    opener = Noop
    viewer = Docx
    cards = ["thumbnail", "file_info"]

    extension = "dj"
    title = "Djot"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Drawio
    extension = "drawio"
    title = "Draw.io"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "123"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "fods"
    title = "Flat XML ODF Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "graffle"
    title = "OmniGraffle"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gslides"
    title = "Google Drive Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = IndesignMarkupLanguage
    extension = "idml"
    title = "Adobe InDesign Markup Language"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdobeIndesignDocument
    extension = "indd"
    title = "Adobe InDesign Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ipynb"
    title = "Interactive Python Notebook"
    category = Document
    indexer = Document_Ipynb
    thumbnailer = Ipynb
    opener = Noop
    viewer = Ipynb
    cards = ["thumbnail", "file_info"]

    extension = "keynote"
    title = "Apple Keynote"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "md"
    title = "Markdown"
    category = Document
    indexer = Document_Md
    thumbnailer = Md
    opener = Noop
    viewer = Markdown
    cards = ["thumbnail", "file_info"]

    format = MicrosoftProjectPlan
    extension = "mpp"
    title = "Microsoft Project Plan"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "nb"
    title = "Mathematica Slideshow"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "nbp"
    title = "Mathematica Player Slideshow"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "numbers"
    title = "Apple Numbers"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = OpendocumentFormula
    extension = "odf"
    title = "OpenDocument Formula"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentGraphics
    extension = "odg"
    title = "OpenDocument Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentTextMaster
    extension = "odm"
    title = "OpenDocument Text Master"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentPresentation
    extension = "odp"
    title = "OpenDocument Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentSpreadsheet
    extension = "ods"
    title = "OpenDocument Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = OpendocumentText
    extension = "odt"
    title = "OpenDocument Text"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentGraphicsTemplate
    extension = "otg"
    title = "OpenDocument Graphics Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentTextMasterTemplate
    extension = "otm"
    title = "OpenDocument Text Master Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OpendocumentSpreadsheetTemplate
    extension = "ots"
    title = "OpenDocument Spreadsheet Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "p7m"
    title = "CieSign Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pages"
    title = "Apple Pages"
    category = Document
    indexer = Noop
    thumbnailer = Mac
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pbix"
    title = "Microsoft PowerBI Report"
    category = Document
    indexer = Document_Pbix
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableDocumentFormat
    extension = "pdf"
    title = "PDF"
    category = Document
    indexer = Document_Pdf
    thumbnailer = Mac
    opener = Noop
    viewer = Pdf
    cards = ["thumbnail", "file_info"]

    extension = "pez"
    title = "Prezi Desktop Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftPowerpointPresentation
    extension = "ppt"
    title = "Microsoft Powerpoint Presentation"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OfficeOpenXmlPresentation
    extension = "pptx"
    title = "Microsoft Powerpoint Presentation"
    category = Document
    indexer = Document_Pptx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "prz"
    title = "Lotus Freelance Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftPublisherDocument
    extension = "pub"
    title = "Microsoft Publisher Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "qpw"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = RichTextFormat
    extension = "rtf"
    title = "Rich Text Format"
    category = Document
    indexer = Document_Rtf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Stardraw
    extension = "sda"
    title = "StarDraw"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Starcalc
    extension = "sdc"
    title = "StarCalc"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Starimpress
    extension = "sdd"
    title = "StarImpress"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Starchart
    extension = "sds"
    title = "StarChart"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Starwriter
    extension = "sdw"
    title = "StarWriter"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlWriterGlobal
    extension = "sgw"
    title = "Sun XML Writer Global"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "shf"
    title = "ThinkFree Show"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Starmath
    extension = "smf"
    title = "StarMath"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlCalcTemplate
    extension = "stc"
    title = "Sun XML Calc Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlDrawTemplate
    extension = "std"
    title = "Sun XML Draw Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlWriterTemplate
    extension = "stw"
    title = "Sun XML Writer Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlCalc
    extension = "sxc"
    title = "Sun XML Calc"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlDraw
    extension = "sxd"
    title = "Sun XML Draw"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlImpress
    extension = "sxi"
    title = "Sun XML Impress"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlImpressTemplate
    extension = "sti"
    title = "Sun XML Impress Template"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlMath
    extension = "sxm"
    title = "Sun XML Math"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SunXmlWriter
    extension = "sxw"
    title = "Sun XML Writer"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "wb1"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wb2"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wb3"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wks"
    title = "Works 1.x-5.x Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wk1"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wk2"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wk3"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wk4"
    title = "Lotus 1-2-3"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wq1"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "wq2"
    title = "Quattro Pro Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "xlr"
    title = "Works 6.x-9.x Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = WordperfectDocument
    extension = "wpd"
    title = "WordPerfect Document"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WordperfectGraphics
    extension = "wpg"
    title = "WordPerfect Graphics"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WordperfectMacro
    extension = "wpm"
    title = "WordPerfect Macro"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Latex
    extension = "tex"
    title = "LaTeX"
    category = Document
    indexer = Noop
    thumbnailer = Tex
    opener = Noop
    viewer = Latex
    cards = ["thumbnail", "file_info"]

    format = PlainText
    extension = "txt"
    title = "Text"
    category = Document
    indexer = Document_Txt
    thumbnailer = Txt
    opener = Noop
    viewer = Text
    cards = ["thumbnail", "file_info"]

    extension = "uos1"
    title = "Uniform Office Format Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "uos2"
    title = "Uniform Office Format Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = MicrosoftVisioDrawing
    extension = "vsd"
    title = "Microsoft Visio Drawing"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OfficeOpenXmlDrawing
    extension = "vsdx"
    title = "Microsoft Visio Drawing"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "webarchive"
    title = "Safari Web Archive"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftExcelSpreadsheet
    extension = "xls"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "xlsb"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    format = OfficeOpenXmlSpreadsheet
    extension = "xlsx"
    title = "Microsoft Excel Spreadsheet"
    category = Document
    indexer = Document_Xlsx
    thumbnailer = Noop
    opener = Noop
    viewer = SheetJS
    cards = ["thumbnail", "file_info"]

    extension = "emlx"
    title = "Apple Email Message"
    category = Email
    indexer = Email_Emlx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "eml"
    title = "Email Message"
    category = Email
    indexer = Email_Eml
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "msg"
    title = "Email Message"
    category = Email
    indexer = Email_Msg
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bdf"
    title = "Glyph Bitmap Distribution Format"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = EmbeddedOpentype
    extension = "eot"
    title = "Embedded OpenType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = BmfontBinary
    extension = "fnt"
    title = "Bitmap Font"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = Opentype
    extension = "otf"
    title = "OpenType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = Truetype
    extension = "ttf"
    title = "TrueType"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = WebOpenFontFormat
    extension = "woff"
    title = "Web Open Font Format"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = WebOpenFontFormat2
    extension = "woff2"
    title = "Web Open Font Format 2"
    category = Font
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Font
    cards = ["thumbnail", "file_info"]

    format = Atari7800Rom
    extension = "a78"
    title = "Atari 7800 ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "acmi"
    title = "TacView Flight Plan"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bms"
    title = "Beat Saber Open Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bsor"
    title = "Beat Saber Open Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "bsp"
    title = "Valve Map"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "craft"
    title = "Kerbal Ship"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "d2s"
    title = "Diablo II Save"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dmx"
    title = "Valve Data Model eXchange"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "eqg"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "fgfp"
    title = "Flight Gear Flight Plan"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GameBoyAdvanceRom
    extension = "gba"
    title = "Game Boy Advance ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GameBoyColorRom
    extension = "gbc"
    title = "Game Boy Color ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GameBoyRom
    extension = "gb"
    title = "Game Boy ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gbx"
    title = "TrackMania Nations Game"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GameGearRom
    extension = "gg"
    title = "Game Gear ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gr2"
    title = "Granny2"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "hltas"
    title = "Half-Life TAS Script"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "jou"
    title = "Othello Players"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "manifest"
    title = "Riot Manifest"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mml"
    title = "Famicon Flavored Music Macro"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "msd"
    title = "StepMania"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mpq"
    title = "Blizzard Data File"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "nav"
    title = "SourceEngine Map"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NeoGeoPocketRom
    extension = "ngp"
    title = "Neo Geo Pocket ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NintendoDsRom
    extension = "nds"
    title = "Nintendo DS ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NintendoEntertainmentSystemRom
    extension = "nes"
    title = "Nintendo Entertainment System ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NintendoSwitchExecutable
    extension = "nso"
    title = "Nintendo Switch Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NintendoSwitchPackage
    extension = "nsp"
    title = "Nintendo Switch Package"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "os"
    title = "osu! Beatmap"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pak"
    title = "Unreal Pak"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pfs"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pgn"
    title = "Chess Portable Game Notation"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "rofl"
    title = "League of Legends Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "s3d"
    title = "Everquest Resource"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sm"
    title = "StepMania"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sc2replay"
    title = "Starcraft 2 Replay"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sqm"
    title = "Arma Mission"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SegaMasterSystemRom
    extension = "sms"
    title = "Sega Master System ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "trn"
    title = "Othello Tournaments"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "ttyrec"
    title = "TTY Recording"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "vdf"
    title = "Valve Data Format"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "vmf"
    title = "Valve Map Format"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "wld"
    title = "Everquest World"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Xbox360Executable
    extension = "xex"
    title = "Xbox 360 Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = XboxExecutable
    extension = "xbe"
    title = "Xbox Executable"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NintendoSwitchRom
    extension = "xci"
    title = "Nintendo Switch ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Nintendo64Rom
    extension = "z64"
    title = "Nintendo 64 ROM"
    category = Game
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Autodesk123d
    extension = "123dx"
    title = "Autodesk 123D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Opennurbs
    extension = "3dm"
    title = "openNURBS"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = ThreeDimensionalStudio
    extension = "3ds"
    title = "3D Studio"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ThreeDimensionalManufacturingFormat
    extension = "3mf"
    title = "3D Manufacturing Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = Model3dAscii
    extension = "a3d"
    title = "Model 3D ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdditiveManufacturingFormat
    extension = "amf"
    title = "Additive Manufacturing Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "ase"
    title = "Aseprite Animation"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "b3d"
    title = "Blitz3d Game Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Blender
    extension = "blend"
    title = "Blender"
    category = Graphics
    indexer = Graphics_Blend
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "c3d"
    title = "C3D Motion Capture"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Cinema4d
    extension = "c4d"
    title = "Cinema 4D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CollaborativeDesignActivity
    extension = "dae"
    title = "Digital Asset Exchange"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GoogleDraco
    extension = "draco"
    title = "Google Draco"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = DesignWebFormat
    extension = "dwf"
    title = "Design Web Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DesignWebFormatXps
    extension = "dwfx"
    title = "Design Web Format XPS"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutocadDrawing
    extension = "dwg"
    title = "AutoCAD Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DrawingExchangeFormatBinary
    extension = "dxf"
    title = "Drawing Exchange Format"
    category = Graphics
    indexer = Graphics_Dxf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "e57"
    title = "Lidar Point Cloud Data"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Fusion360
    extension = "f3d"
    title = "Fusion 360"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Filmbox
    extension = "fbx"
    title = "Filmbox"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "gldf"
    title = "Global Lighting Data Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "glif"
    title = "Glyph Interchange Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GlTransmissionFormatBinary
    extension = "glb"
    title = "GL Transmission Format Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "gltf"
    title = "GL Transmission Format Text"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "gvox"
    title = "General Voxel Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskInventorAssembly
    extension = "iam"
    title = "Autodesk Inventor Assembly"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskInventorDrawing
    extension = "idw"
    title = "Autodesk Inventor Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = InitialGraphicsExchangeSpecification
    extension = "iges"
    title = "Initial Graphics Exchange Specification"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskInventorPresentation
    extension = "ipn"
    title = "Autodesk Inventor Presentation"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskInventorPart
    extension = "ipt"
    title = "Autodesk Inventor Part"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = InterQuakeExport
    extension = "iqe"
    title = "Inter-Quake Export"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = InterQuakeModel
    extension = "iqm"
    title = "Inter-Quake Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = KhronosTexture
    extension = "ktx"
    title = "Khronos Texture"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = KhronosTexture2
    extension = "ktx2"
    title = "Khronos Texture 2"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "lw02"
    title = "Lightwave 3D Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = Model3dBinary
    extension = "m3d"
    title = "Model 3D Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MayaAscii
    extension = "ma"
    title = "Maya ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ThreeDimensionalStudioMax
    extension = "max"
    title = "3D Studio Max"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MayaBinary
    extension = "mb"
    title = "Maya Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mmd"
    title = "MikuMikuDance Model"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "msh"
    title = "Gmsh Mesh"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mtl"
    title = "Wavefront Material Library"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "obj"
    title = "Wavefront Object"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "off"
    title = "Mesh Object File Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PolygonBinary
    extension = "ply"
    title = "Polygon"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "pxr"
    title = "Pixar Image Format"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "rmesh"
    title = "Room Mesh"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SolidworksAssembly
    extension = "sldasm"
    title = "SolidWorks Assembly"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SolidworksDrawing
    extension = "slddrw"
    title = "SolidWorks Drawing"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SolidworksPart
    extension = "sldprt"
    title = "SolidWorks Part"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SpaceclaimDocument
    extension = "scdoc"
    title = "SpaceClaim Document"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "sdf"
    title = "Simulation Description Format"
    category = Graphics
    indexer = Hardware_Sdf
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = StandardForTheExchangeOfProductModelData
    extension = "step"
    title = "Standard for the Exchange of Products"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Sketchup
    extension = "skp"
    title = "SketchUp"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = StereolithographyAscii
    extension = "stl"
    title = "Stereolithography ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = Universal3d
    extension = "u3d"
    title = "Universal 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UniversalSceneDescriptionAscii
    extension = "usda"
    title = "Universal Scene Description ASCII"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UniversalSceneDescriptionBinary
    extension = "usdc"
    title = "Universal Scene Description Binary"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = UniversalSceneDescriptionZip
    extension = "usdz"
    title = "Universal Scene Description Zip"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = VirtualRealityModelingLanguage
    extension = "vrml"
    title = "Virtual Reality Modeling Language"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = Magicavoxel
    extension = "vox"
    title = "MagicaVoxel"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = AutodeskAlias
    extension = "wire"
    title = "Autodesk Alias"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Extensible3d
    extension = "x3d"
    title = "Extensible 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Extensible3d
    extension = "x3db"
    title = "Extensible 3D"
    category = Graphics
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cir"
    title = "Spice Hardware Description"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "fld"
    title = "Ansys 3D Fields"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gdb"
    title = "Garmin GPS Database"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gds"
    title = "GDSII File Format"
    category = Hardware
    indexer = Hardware_Gds
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "lef"
    title = "Library Exchange Format"
    category = Hardware
    indexer = Hardware_Lef
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "veryl"
    title = "Veryl Hardware Description"
    category = Hardware
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "3fr"
    title = "Hasselblad Raw"
    category = Image
    indexer = Noop
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "aai"
    title = "AAI Dune"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "art"
    title = "PFS: 1st Publisher"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "arw"
    title = "Sony Alpha Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdaptableScalableTextureCompression
    extension = "astc"
    title = "Adaptable Scalable Texture Compression"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdobeIllustratorArtwork
    extension = "ai"
    title = "Adobe Illustrator Artwork"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AnimatedPortableNetworkGraphics
    extension = "apng"
    title = "Animated Portable Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsAnimatedCursor
    extension = "avaniif"
    title = "Windows Animated Cursor"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Av1ImageFileFormat
    extension = "avif"
    title = "AV1 Image File Format"
    category = Image
    indexer = Noop
    thumbnailer = Avif
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Av1ImageFileFormatSequence
    extension = "avifs"
    title = "AV1 Image File Format Sequence"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "avs"
    title = "AVS X"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsBitmap
    extension = "bmp"
    title = "Windows Bitmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Bmp
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = BetterPortableGraphics
    extension = "bpg"
    title = "Better Portable Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "brf"
    title = "Braille Ready Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "cals"
    title = "Continuous Acquisition and Life-cycle Support"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Cineon
    extension = "cin"
    title = "Cineon"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CanonRaw
    extension = "crw"
    title = "Canon Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CanonRaw2
    extension = "cr2"
    title = "Canon Raw 2"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = CanonRaw3
    extension = "cr3"
    title = "Canon Raw 3"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsCursor
    extension = "cur"
    title = "Windows Cursor"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DigitalImagingAndCommunicationsInMedicine
    extension = "dcm"
    title = "Digital Imaging and Communications in Medicine "
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dcr"
    title = "Kodak Digital Camera Raw "
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftDirectdrawSurface
    extension = "dds"
    title = "Microsoft DirectDraw Surface"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Dds
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    extension = "dng"
    title = "Adobe DNG Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dot"
    title = "Graph Visualization"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "dpx"
    title = "SMPTE Digital Moving Picture Exchange"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Djvu
    extension = "dvju"
    title = "DjVu"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = DigitalPictureExchange
    extension = "dpx"
    title = "Digital Picture Exchange"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "emf"
    title = "Microsoft Enhanced Metafile"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = EncapsulatedPostscript
    extension = "eps"
    title = "Encapsulated PostScript"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Openexr
    extension = "exr"
    title = "OpenEXR"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Three
    cards = ["thumbnail", "file_info"]

    format = ExtensibleStylesheetLanguageTransformations
    extension = "ff"
    title = "Farbfeld"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FreeLosslessImageFormat
    extension = "flif"
    title = "Free Lossless Image Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "fpx"
    title = "FlashPix"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GraphicsInterchangeFormat
    extension = "gif"
    title = "Graphics Interchange Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Gif
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "gpmf"
    title = "GoPro Metadata Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = RadianceHdr
    extension = "hdr"
    title = "Radiance HDR"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = HighEfficiencyImageCoding
    extension = "heic"
    title = "High Efficiency Image Coding"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Heif
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = HighEfficiencyImageCodingSequence
    extension = "heics"
    title = "High Efficiency Image Coding Sequence"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = HighEfficiencyImageFileFormat
    extension = "heif"
    title = "High Efficiency Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Heif
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = HighEfficiencyImageFileFormatSequence
    extension = "heifs"
    title = "High Efficiency Image File Format Sequence"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "hvif"
    title = "Haiku Vector Icon Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = IccProfile
    extension = "icc"
    title = "ICC Profile"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleIconImage
    extension = "icns"
    title = "Apple Icon Image"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsIcon
    extension = "ico"
    title = "Windows Icon"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Ico
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Jpeg2000Codestream
    extension = "j2c"
    title = "JPEG 2000 Codestream"
    category = Image
    indexer = Noop
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JpegLs
    extension = "jls"
    title = "JPEG-LS"
    category = Image
    indexer = Noop
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JpegNetworkGraphics
    extension = "jng"
    title = "JPEG Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Jpeg2000Part1
    extension = "jp2"
    title = "JPEG 2000 Part 1"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JointPhotographicExpertsGroup
    extension = "jpeg"
    title = "JPEG"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JointPhotographicExpertsGroup
    extension = "jpg"
    title = "JPEG"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Jpeg2000Part6
    extension = "jpm"
    title = "JPEG 2000 Part 6"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Jpeg2000Part2
    extension = "jpx"
    title = "JPEG 2000 Part 2"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JpegExtendedRange
    extension = "jxr"
    title = "JPEG Extended Range"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = JpegXl
    extension = "jxl"
    title = "JPEG-XL"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Jpeg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mat"
    title = "MATLAB Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MagickImageFileFormat
    extension = "miff"
    title = "Magick Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MultipleImageNetworkGraphics
    extension = "mng"
    title = "Multiple-image Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mrw"
    title = "Sony (Minolta) Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mtv"
    title = "MTV Raytracing"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = NikonElectronicFile
    extension = "nef"
    title = "Nikon Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Openraster
    extension = "ora"
    title = "OpenRaster"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OlympusRawFormat
    extension = "orf"
    title = "Olympus Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableArbitraryMap
    extension = "pam"
    title = "Portable Arbitrary Map"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableBitmap
    extension = "pbm"
    title = "Portable Bitmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "pef"
    title = "Pentax Electronic File"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableFloatmap
    extension = "pfm"
    title = "Portable Floatmap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableGraymap
    extension = "pgm"
    title = "Portable Graymap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortableNetworkGraphics
    extension = "png"
    title = "Portable Network Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Png
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PortablePixmap
    extension = "ppm"
    title = "Portable PixMap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Ppm
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Postscript
    extension = "ps"
    title = "PostScript"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AdobePhotoshopDocument
    extension = "psd"
    title = "Adobe Photoshop"
    category = Image
    indexer = Image_Psd
    thumbnailer = Psd
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = QuiteOkImage
    extension = "qoi"
    title = "Quick OK Image"
    category = Image
    indexer = Noop
    thumbnailer = Qoi
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FujifilmRaw
    extension = "raf"
    title = "Fujifilm Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = PanasonicRaw
    extension = "rw2"
    title = "Panasonic Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "rwl"
    title = "Leica Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SiliconGraphicsImage
    extension = "sgi"
    title = "Silicon Graphics Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ScalableVectorGraphics
    extension = "svg"
    title = "Scalable Vector Graphics"
    category = Image
    indexer = Image_Svg
    thumbnailer = Svg
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tga"
    title = "Truevision Targa Image"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tga
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "tif"
    title = "Tag Image File Format"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tiff
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = TagImageFileFormat
    extension = "tiff"
    title = "TIFF"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Tiff
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ExperimentalComputingFacility
    extension = "xcf"
    title = "Experimental Computing Facility"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Webp
    extension = "webp"
    title = "WebP"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Webp
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsMetafile
    extension = "wmf"
    title = "Windows Metafile"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "wpg"
    title = "Word Perfect Graphics"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "x3f"
    title = "Sigma Raw"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Raw
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "xisf"
    title = "Extensible Image Serialization Format"
    category = Image
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = XPixmap
    extension = "xpm"
    title = "X PixMap"
    category = Image
    indexer = Image_Imagemagick
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlexibleAndInteroperableDataTransfer
    extension = "fit"
    title = "Flexible and Interoperable Data Transfer"
    category = Map
    indexer = Map_Fit
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlexibleImageTransportSystem
    extension = "fits"
    title = "Flexible Image Transport System"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = GpsExchangeFormat
    extension = "gpx"
    title = "GPS Exchange Format"
    category = Map
    indexer = Map_Gpx
    thumbnailer = Noop
    opener = Noop
    viewer = Leaflet
    cards = ["thumbnail", "file_info"]

    format = GeographyMarkupLanguage
    extension = "gml"
    title = "Geography Markup Language"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = KeyholeMarkupLanguage
    extension = "kml"
    title = "Keyhole Markup Language"
    category = Map
    indexer = Map_Kml
    thumbnailer = Noop
    opener = Noop
    viewer = Leaflet
    cards = ["thumbnail", "file_info"]

    format = KeyholeMarkupLanguageZip
    extension = "kmz"
    title = "Keyhole Markup Language Zip"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "loc"
    title = "GPS Location"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "osrm"
    title = "OpenStreetMap Route"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "rinex"
    title = "RTCM Receiver Independent Exchange"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Shapefile
    extension = "shp"
    title = "Shapefile"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = TiledTilesetXml
    extension = "tsx"
    title = "Tiled Tileset XML"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = TimedTextMarkupLanguage
    extension = "ttml"
    title = "Timed Text Markup Language"
    category = Map
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = TrainingCenterXml
    extension = "tcx"
    title = "Training Center XML"
    category = Map
    indexer = Map_Tcx
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ThirdGenerationPartnershipProject
    extension = "3gp"
    title = "3rd Generation Partnership Project"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ThirdGenerationPartnershipProject
    extension = "3gpp"
    title = "3rd Generation Partnership Project"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ThirdGenerationPartnershipProject2
    extension = "3gpp2"
    title = "3rd Generation Partnership Project 2"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = ActionsMediaVideo
    extension = "amv"
    title = "Actions Media Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AudioVideoInterleave
    extension = "avi"
    title = "Audio Video Interleave"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MicrosoftDigitalVideoRecording
    extension = "dvr-ms"
    title = "Microsoft Digital Video Recording"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlashMp4ProtectedVideo
    extension = "f4p"
    title = "Adobe Flash Player Protected Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlashMp4Video
    extension = "f4v"
    title = "Adobe Flash Player Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskAnimatorPro
    extension = "flc"
    title = "Autodesk Animator Pro"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AutodeskAnimator
    extension = "fli"
    title = "Autodesk Animator"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = FlashVideo
    extension = "flv"
    title = "Adobe Flash Player Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = BdavMpeg2TransportStream
    extension = "m2ts"
    title = "MPEG-2 Transport Stream"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleItunesVideo
    extension = "m4v"
    title = "Apple iTunes Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Jpeg2000Part3
    extension = "mj2"
    title = "JPEG 2000 Part 3"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Matroska3dVideo
    extension = "mk3d"
    title = "Matroska 3D Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MatroskaVideo
    extension = "mkv"
    title = "Matroska Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = AppleQuicktime
    extension = "mov"
    title = "Apple QuickTime Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Mpeg12Video
    extension = "mpg"
    title = "MPEG-1/2 Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "mpls"
    title = "Movie Playlist"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

   format = Mpeg4Part14Video
    extension = "mp4"
    title = "MPEG-4 Part 14 Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Mtv
    extension = "mtv"
    title = "MTV"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = SonyMovie
    extension = "mqv"
    title = "Sony Movie"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = MaterialExchangeFormat
    extension = "mxf"
    title = "Material Exchange Format"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggMedia
    extension = "ogm"
    title = "Ogg Media"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = OggTheora
    extension = "ogv"
    title = "Ogg Theora"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Realvideo
    extension = "rv"
    title = "RealVideo"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "srt"
    title = "SubRip Subtitles"
    category = Video
    indexer = Video_Srt
    thumbnailer = Noop
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    extension = "vcxproj"
    title = "Visual Studio Project"
    category = Code
    indexer = Noop
    thumbnailer = Noop
    opener = Noop
    viewer = Code
    cards = ["thumbnail", "file_info"]

    extension = "vob"
    title = "VOB"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = Webm
    extension = "webm"
    title = "WebM"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsMediaVideo
    extension = "wmv"
    title = "Windows Media Video"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]

    format = WindowsRecordedTvShow
    extension = "wtv"
    title = "Windows Recorded TV Show"
    category = Video
    indexer = Video_Ffmpeg
    thumbnailer = Video
    opener = Noop
    viewer = Noop
    cards = ["thumbnail", "file_info"]
}