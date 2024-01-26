import Foundation
import AppKit
import SwiftRs
import QuickLookThumbnailing

func convertTo8BitColorPNG(image: NSImage) -> SRString {
    guard let cgImage = image.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
        print("Failed to get CGImage from NSImage")
        return SRString()
    }

    let width = cgImage.width
    let height = cgImage.height
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue)

    let context = CGContext(data: nil,
                               width: width,
                               height: height,
                               bitsPerComponent: 8,
                               bytesPerRow: width * 4,
                               space: colorSpace,
                               bitmapInfo: bitmapInfo.rawValue)!

    context.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))
    let rep = NSBitmapImageRep(cgImage: context.makeImage()!)
    return SRString(rep.representation(using: .tiff, properties: [:])!.base64EncodedString())
}

@_cdecl("file_thumbnail")
public func fileThumbnail(source_path: SRString) -> SRString {
    let source_path = source_path.toString()

    let image = NSWorkspace.shared.icon(forFile: source_path)
    let bitmap = NSBitmapImageRep(data: image.tiffRepresentation!)!.representation(using: .png, properties: [:])!

    return SRString(bitmap.base64EncodedString())
}

@_cdecl("quicklook_preview")
@available(macOS 10.15, *)
public func quicklookPreview(source_path: SRString) -> SRString {
    let source_path = URL(fileURLWithPath: source_path.toString())
    let scale = AppKit.NSScreen.main?.backingScaleFactor

    let request = QLThumbnailGenerator.Request(fileAt: source_path,
                                               size: CGSize(width: 800, height: 800),
                                               scale: scale!,
                                               representationTypes: .thumbnail)

    let semaphore = DispatchSemaphore(value: 0)
    var base64 = SRString()

    let completionHandler: (QLThumbnailRepresentation?, Error?) -> Void = {
        thumbnail, error in

        if (error != nil) {
            print("Failed to generate thumbnail due to: " + error!.localizedDescription)
        }

        if (thumbnail != nil) {
            base64 = convertTo8BitColorPNG(image: thumbnail!.nsImage)
        }
        semaphore.signal()
    }
    QLThumbnailGenerator.shared.generateBestRepresentation(for: request, completion: completionHandler)
    semaphore.wait()
    return base64
}
