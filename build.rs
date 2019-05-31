use cc;

fn main() {
    // \
    //    \
    //
    //    \
    //    \
    //

    // src/libiconv-1.15/lib/iconv.c \
    //	src/libiconv-1.15/libcharset/lib/localcharset.c \
    //

    cc::Build::new()
        .flag_if_supported("-Wmissing-field-initializers")
        .flag_if_supported("-Wno-multichar")
        .flag_if_supported("-D_ANDROID ")
        .flag_if_supported("-DLIBDIR=\"c\"")
        .flag_if_supported("-DBUILDING_LIBICONV")
        .flag_if_supported("-DBUILDING_LIBCHARSET")
        .flag_if_supported("-DIN_LIBRARY")
        .file("src/libiconv-1.15/lib/iconv.c")
        .file("src/libiconv-1.15/libcharset/lib/localcharset.c")
        .file("src/libiconv-1.15/lib/relocatable.c")
        .include("src/libiconv-1.15/include")
        .include("src/libiconv-1.15/libcharset")
        .include("src/libiconv-1.15/libcharset/include")

        .file("src/zbar/img_scanner.c")
        .file("src/zbar/decoder.c")
        .file("src/zbar/image.c")
        .file("src/zbar/symbol.c")
        .file("src/zbar/convert.c")
        .file("src/zbar/config.c")
        .file("src/zbar/scanner.c")
        .file("src/zbar/error.c")
        .file("src/zbar/refcnt.c")
        .file("src/zbar/video.c")
        .file("src/zbar/video/null.c")
        .file("src/zbar/decoder/code128.c")
        .file("src/zbar/decoder/code39.c")
        .file("src/zbar/decoder/code93.c")
        .file("src/zbar/decoder/codabar.c")
        .file("src/zbar/decoder/databar.c")
        .file("src/zbar/decoder/ean.c")
        .file("src/zbar/decoder/i25.c")
        .file("src/zbar/decoder/qr_finder.c")
        .file("src/zbar/qrcode/bch15_5.c")
        .file("src/zbar/qrcode/binarize.c")
        .file("src/zbar/qrcode/isaac.c")
        .file("src/zbar/qrcode/qrdec.c")
        .file("src/zbar/qrcode/qrdectxt.c")
        .file("src/zbar/qrcode/rs.c")
        .file("src/zbar/qrcode/util.c")
        .include("src/libiconv-1.15/include")
        .include("src/include")
        .include("src/zbar")
        .compile("zbar");
}