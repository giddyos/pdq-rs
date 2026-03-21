#include "pdq_wrapper.h"
#include "hashing/pdqhashing.h"
#include "common/pdqhashtypes.h"
#include "common/pdqhamming.h"
#include <cstring>
#include <vector>

// all free functions live in this namespace
using namespace facebook::pdq::hashing;

static void copy_hash(const Hash256 &src, PdqHash256 *dst)
{
    if (!dst) {
        return;
    }

    const uint8_t *src_bytes = reinterpret_cast<const uint8_t *>(src.w);
    for (int i = 0; i < 32; ++i) {
        dst->bits[31 - i] = src_bytes[i];
    }
}

int pdq_hash_from_rgb(
    const uint8_t *rgb,
    int num_rows,
    int num_cols,
    PdqHash256 *out_hash)
{
    std::vector<float> luma(num_rows * num_cols);
    std::vector<float> buf1(num_rows * num_cols);

    fillFloatLumaFromRGB(
        const_cast<uint8_t *>(rgb),
        const_cast<uint8_t *>(rgb + 1),
        const_cast<uint8_t *>(rgb + 2),
        num_rows, num_cols,
        num_cols * 3,
        3,
        luma.data());

    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];

    Hash256 hash;
    int quality = 0;

    pdqHash256FromFloatLuma(
        luma.data(),
        buf1.data(),
        num_rows, num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        hash,
        quality);

    copy_hash(hash, out_hash);
    return quality;
}

int pdq_hash_from_grey(
    const uint8_t *grey,
    int num_rows,
    int num_cols,
    PdqHash256 *out_hash)
{
    std::vector<float> luma(num_rows * num_cols);
    std::vector<float> buf1(num_rows * num_cols);

    fillFloatLumaFromGrey(
        const_cast<uint8_t *>(grey),
        num_rows, num_cols,
        num_cols,
        1,
        luma.data());

    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];

    Hash256 hash;
    int quality = 0;
    pdqHash256FromFloatLuma(
        luma.data(), buf1.data(),
        num_rows, num_cols,
        buffer64x64, buffer16x64, buffer16x16,
        hash, quality);
    copy_hash(hash, out_hash);
    return quality;
}

int pdq_dihedral_hash_from_rgb(
    const uint8_t *rgb,
    int num_rows,
    int num_cols,
    PdqHash256 *out_original,
    PdqHash256 *out_rotate90,
    PdqHash256 *out_rotate180,
    PdqHash256 *out_rotate270,
    PdqHash256 *out_flip_x,
    PdqHash256 *out_flip_y,
    PdqHash256 *out_flip_plus1,
    PdqHash256 *out_flip_minus1)
{
    std::vector<float> luma(num_rows * num_cols);
    std::vector<float> buf1(num_rows * num_cols);

    fillFloatLumaFromRGB(
        const_cast<uint8_t *>(rgb),
        const_cast<uint8_t *>(rgb + 1),
        const_cast<uint8_t *>(rgb + 2),
        num_rows, num_cols,
        num_cols * 3,
        3,
        luma.data());

    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];

    // extra buffer needed for dihedral
    float buffer16x16Aux[16][16];

    Hash256 h0, h90, h180, h270, hfx, hfy, hfp1, hfm1;

    int quality = 0;
    pdqDihedralHash256esFromFloatLuma(
        luma.data(),
        buf1.data(),
        num_rows, num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        buffer16x16Aux,
        out_original ? &h0 : nullptr,
        out_rotate90 ? &h90 : nullptr,
        out_rotate180 ? &h180 : nullptr,
        out_rotate270 ? &h270 : nullptr,
        out_flip_x ? &hfx : nullptr,
        out_flip_y ? &hfy : nullptr,
        out_flip_plus1 ? &hfp1 : nullptr,
        out_flip_minus1 ? &hfm1 : nullptr,
        quality);

    if (out_original)
        copy_hash(h0, out_original);
    if (out_rotate90)
        copy_hash(h90, out_rotate90);
    if (out_rotate180)
        copy_hash(h180, out_rotate180);
    if (out_rotate270)
        copy_hash(h270, out_rotate270);
    if (out_flip_x)
        copy_hash(hfx, out_flip_x);
    if (out_flip_y)
        copy_hash(hfy, out_flip_y);
    if (out_flip_plus1)
        copy_hash(hfp1, out_flip_plus1);
    if (out_flip_minus1)
        copy_hash(hfm1, out_flip_minus1);

    return quality;
}

int pdq_dihedral_hash_from_grey(
    const uint8_t *grey,
    int num_rows,
    int num_cols,
    PdqHash256 *out_original,
    PdqHash256 *out_rotate90,
    PdqHash256 *out_rotate180,
    PdqHash256 *out_rotate270,
    PdqHash256 *out_flip_x,
    PdqHash256 *out_flip_y,
    PdqHash256 *out_flip_plus1,
    PdqHash256 *out_flip_minus1)
{
    std::vector<float> luma(num_rows * num_cols);
    std::vector<float> buf1(num_rows * num_cols);

    fillFloatLumaFromGrey(
        const_cast<uint8_t *>(grey),
        num_rows, num_cols,
        num_cols,
        1,
        luma.data());

    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];
    float buffer16x16Aux[16][16];

    Hash256 h0, h90, h180, h270, hfx, hfy, hfp1, hfm1;

    int quality = 0;
    pdqDihedralHash256esFromFloatLuma(
        luma.data(),
        buf1.data(),
        num_rows, num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        buffer16x16Aux,
        out_original ? &h0 : nullptr,
        out_rotate90 ? &h90 : nullptr,
        out_rotate180 ? &h180 : nullptr,
        out_rotate270 ? &h270 : nullptr,
        out_flip_x ? &hfx : nullptr,
        out_flip_y ? &hfy : nullptr,
        out_flip_plus1 ? &hfp1 : nullptr,
        out_flip_minus1 ? &hfm1 : nullptr,
        quality);

    if (out_original)
        copy_hash(h0, out_original);
    if (out_rotate90)
        copy_hash(h90, out_rotate90);
    if (out_rotate180)
        copy_hash(h180, out_rotate180);
    if (out_rotate270)
        copy_hash(h270, out_rotate270);
    if (out_flip_x)
        copy_hash(hfx, out_flip_x);
    if (out_flip_y)
        copy_hash(hfy, out_flip_y);
    if (out_flip_plus1)
        copy_hash(hfp1, out_flip_plus1);
    if (out_flip_minus1)
        copy_hash(hfm1, out_flip_minus1);

    return quality;
}

static inline int popcount8(uint8_t x)
{
    int n = 0;
    while (x) {
        x &= static_cast<uint8_t>(x - 1);
        ++n;
    }
    return n;
}

int pdq_hamming_distance(const PdqHash256 *a, const PdqHash256 *b)
{
    if (!a || !b) {
        return -1;
    }

    int dist = 0;
    for (int i = 0; i < 32; ++i) {
        dist += popcount8(static_cast<uint8_t>(a->bits[i] ^ b->bits[i]));
    }
    return dist;
}
