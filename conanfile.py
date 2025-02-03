from conan import ConanFile

class Harana(ConanFile):
    name = "harana"
    version = "1.0"

    def requirements(self):
        self.requires("ffmpeg/6.1")
        self.requires("ktx/4.0.0")
        self.requires("leptonica/1.83.1")
        self.requires("libarchive/3.7.2")
        self.requires("libheif/1.16.2")
        self.requires("libmp3lame/3.100")
        self.requires("libpng/1.6.42", override=True)
        self.requires("libraw/0.21.1")
        self.requires("openjpeg/2.5.2")
        self.requires("tesseract/5.3.0")
