#include <dng_host.h>
#include <dng_image.h>
#include <dng_file_stream.h>
#include <dng_xmp.h>
#include <dng_image_writer.h>
#include <dng_big_table.h>
#include <iostream>
#include <sstream>
#include <fstream>
#include <string>

extern "C" {
    int dng_read() {
        dng_host host;

        // Read the TIFF image from XMP table
        std::ifstream data("/mnt/c/Users/david/Downloads/6M_0006.xmp");
        std::stringstream buffer;
        buffer << data.rdbuf();
        dng_xmp_sdk::InitializeSDK();
        dng_xmp xmp(host.Allocator());
        xmp.Parse(host, buffer.str().c_str(), buffer.str().length());
        dng_image_table table;
        dng_fingerprint fingerprint("408439BE9D27662CA2153F95DD978032");
        table.ReadTableFromXMP(xmp, "http://ns.adobe.com/camera-raw-settings/1.0/", fingerprint);
        auto image = table.ShareImage();
        dng_file_stream stream("/mnt/c/Users/david/Downloads/K62_2602.tiff", true);
        dng_image_writer writer;
        writer.WriteTIFF(host, stream, *image, image->Planes() >= 3 ? piRGB : piBlackIsZero);

        dng_image_table table2;
        dng_image_table_jxl_compression_info comp_info;
        table2.SetImage(image.get(), &comp_info);
        dng_big_table_storage storage;
        dng_xmp xmp2(host.Allocator());
        table2.WriteToXMP(xmp2, "http://ns.adobe.com/camera-raw-settings/1.0/", "Table_", storage);
        auto *result = xmp2.Serialize(true);
        auto *buffer2 = result->Buffer_char();
        std::string res_string(buffer2);
        std::cout << res_string;
        return 0;
    }
}
