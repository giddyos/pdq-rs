#include "pdq_wrapper.h"
#include "hashing/pdqhashing.h"
#include "common/pdqhashtypes.h"
#include "common/pdqhamming.h"

#include <stdint.h>
#include <stdlib.h>

static int alloc_float_buffers(
    int num_rows,
    int num_cols,
    float **out_luma,
    float **out_buf1)
{
    size_t count;
    float *luma;
    float *buf1;

    if (out_luma == NULL || out_buf1 == NULL)
    {
        return 0;
    }

    if (num_rows <= 0 || num_cols <= 0)
    {
        return 0;
    }

    count = (size_t)num_rows * (size_t)num_cols;

    if (count == 0)
    {
        return 0;
    }

    luma = (float *)malloc(sizeof(float) * count);
    if (luma == NULL)
    {
        return 0;
    }

    buf1 = (float *)malloc(sizeof(float) * count);
    if (buf1 == NULL)
    {
        free(luma);
        return 0;
    }

    *out_luma = luma;
    *out_buf1 = buf1;
    return 1;
}

static void free_float_buffers(float *luma, float *buf1)
{
    if (buf1 != NULL)
    {
        free(buf1);
    }
    if (luma != NULL)
    {
        free(luma);
    }
}

static void copy_hash(
    const facebook::pdq::hashing::Hash256 *src,
    PdqHash256 *dst)
{
    const uint8_t *src_bytes;
    int i;

    if (src == NULL || dst == NULL)
    {
        return;
    }

    src_bytes = (const uint8_t *)(src->w);
    for (i = 0; i < 32; ++i)
    {
        dst->bits[31 - i] = src_bytes[i];
    }
}

int pdq_hash_from_rgb(
    const uint8_t *rgb,
    int num_rows,
    int num_cols,
    PdqHash256 *out_hash)
{
    float *luma;
    float *buf1;
    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];
    facebook::pdq::hashing::Hash256 hash;
    int quality;

    if (rgb == NULL || out_hash == NULL)
    {
        return -1;
    }

    if (!alloc_float_buffers(num_rows, num_cols, &luma, &buf1))
    {
        return -1;
    }

    facebook::pdq::hashing::fillFloatLumaFromRGB(
        (uint8_t *)rgb,
        (uint8_t *)(rgb + 1),
        (uint8_t *)(rgb + 2),
        num_rows,
        num_cols,
        num_cols * 3,
        3,
        luma);

    quality = 0;

    facebook::pdq::hashing::pdqHash256FromFloatLuma(
        luma,
        buf1,
        num_rows,
        num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        hash,
        quality);

    copy_hash(&hash, out_hash);
    free_float_buffers(luma, buf1);
    return quality;
}

int pdq_hash_from_grey(
    const uint8_t *grey,
    int num_rows,
    int num_cols,
    PdqHash256 *out_hash)
{
    float *luma;
    float *buf1;
    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];
    facebook::pdq::hashing::Hash256 hash;
    int quality;

    if (grey == NULL || out_hash == NULL)
    {
        return -1;
    }

    if (!alloc_float_buffers(num_rows, num_cols, &luma, &buf1))
    {
        return -1;
    }

    facebook::pdq::hashing::fillFloatLumaFromGrey(
        (uint8_t *)grey,
        num_rows,
        num_cols,
        num_cols,
        1,
        luma);

    quality = 0;

    facebook::pdq::hashing::pdqHash256FromFloatLuma(
        luma,
        buf1,
        num_rows,
        num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        hash,
        quality);

    copy_hash(&hash, out_hash);
    free_float_buffers(luma, buf1);
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
    float *luma;
    float *buf1;
    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];
    float buffer16x16Aux[16][16];
    facebook::pdq::hashing::Hash256 h0;
    facebook::pdq::hashing::Hash256 h90;
    facebook::pdq::hashing::Hash256 h180;
    facebook::pdq::hashing::Hash256 h270;
    facebook::pdq::hashing::Hash256 hfx;
    facebook::pdq::hashing::Hash256 hfy;
    facebook::pdq::hashing::Hash256 hfp1;
    facebook::pdq::hashing::Hash256 hfm1;
    int quality;

    if (rgb == NULL)
    {
        return -1;
    }

    if (!alloc_float_buffers(num_rows, num_cols, &luma, &buf1))
    {
        return -1;
    }

    facebook::pdq::hashing::fillFloatLumaFromRGB(
        (uint8_t *)rgb,
        (uint8_t *)(rgb + 1),
        (uint8_t *)(rgb + 2),
        num_rows,
        num_cols,
        num_cols * 3,
        3,
        luma);

    quality = 0;

    facebook::pdq::hashing::pdqDihedralHash256esFromFloatLuma(
        luma,
        buf1,
        num_rows,
        num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        buffer16x16Aux,
        out_original ? &h0 : NULL,
        out_rotate90 ? &h90 : NULL,
        out_rotate180 ? &h180 : NULL,
        out_rotate270 ? &h270 : NULL,
        out_flip_x ? &hfx : NULL,
        out_flip_y ? &hfy : NULL,
        out_flip_plus1 ? &hfp1 : NULL,
        out_flip_minus1 ? &hfm1 : NULL,
        quality);

    if (out_original)
    {
        copy_hash(&h0, out_original);
    }
    if (out_rotate90)
    {
        copy_hash(&h90, out_rotate90);
    }
    if (out_rotate180)
    {
        copy_hash(&h180, out_rotate180);
    }
    if (out_rotate270)
    {
        copy_hash(&h270, out_rotate270);
    }
    if (out_flip_x)
    {
        copy_hash(&hfx, out_flip_x);
    }
    if (out_flip_y)
    {
        copy_hash(&hfy, out_flip_y);
    }
    if (out_flip_plus1)
    {
        copy_hash(&hfp1, out_flip_plus1);
    }
    if (out_flip_minus1)
    {
        copy_hash(&hfm1, out_flip_minus1);
    }

    free_float_buffers(luma, buf1);
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
    float *luma;
    float *buf1;
    float buffer64x64[64][64];
    float buffer16x64[16][64];
    float buffer16x16[16][16];
    float buffer16x16Aux[16][16];
    facebook::pdq::hashing::Hash256 h0;
    facebook::pdq::hashing::Hash256 h90;
    facebook::pdq::hashing::Hash256 h180;
    facebook::pdq::hashing::Hash256 h270;
    facebook::pdq::hashing::Hash256 hfx;
    facebook::pdq::hashing::Hash256 hfy;
    facebook::pdq::hashing::Hash256 hfp1;
    facebook::pdq::hashing::Hash256 hfm1;
    int quality;

    if (grey == NULL)
    {
        return -1;
    }

    if (!alloc_float_buffers(num_rows, num_cols, &luma, &buf1))
    {
        return -1;
    }

    facebook::pdq::hashing::fillFloatLumaFromGrey(
        (uint8_t *)grey,
        num_rows,
        num_cols,
        num_cols,
        1,
        luma);

    quality = 0;

    facebook::pdq::hashing::pdqDihedralHash256esFromFloatLuma(
        luma,
        buf1,
        num_rows,
        num_cols,
        buffer64x64,
        buffer16x64,
        buffer16x16,
        buffer16x16Aux,
        out_original ? &h0 : NULL,
        out_rotate90 ? &h90 : NULL,
        out_rotate180 ? &h180 : NULL,
        out_rotate270 ? &h270 : NULL,
        out_flip_x ? &hfx : NULL,
        out_flip_y ? &hfy : NULL,
        out_flip_plus1 ? &hfp1 : NULL,
        out_flip_minus1 ? &hfm1 : NULL,
        quality);

    if (out_original)
    {
        copy_hash(&h0, out_original);
    }
    if (out_rotate90)
    {
        copy_hash(&h90, out_rotate90);
    }
    if (out_rotate180)
    {
        copy_hash(&h180, out_rotate180);
    }
    if (out_rotate270)
    {
        copy_hash(&h270, out_rotate270);
    }
    if (out_flip_x)
    {
        copy_hash(&hfx, out_flip_x);
    }
    if (out_flip_y)
    {
        copy_hash(&hfy, out_flip_y);
    }
    if (out_flip_plus1)
    {
        copy_hash(&hfp1, out_flip_plus1);
    }
    if (out_flip_minus1)
    {
        copy_hash(&hfm1, out_flip_minus1);
    }

    free_float_buffers(luma, buf1);
    return quality;
}

static int popcount8(uint8_t x)
{
    int n = 0;
    while (x)
    {
        x &= (uint8_t)(x - 1);
        ++n;
    }
    return n;
}

int pdq_hamming_distance(const PdqHash256 *a, const PdqHash256 *b)
{
    int dist;
    int i;

    if (a == NULL || b == NULL)
    {
        return -1;
    }

    dist = 0;
    for (i = 0; i < 32; ++i)
    {
        dist += popcount8((uint8_t)(a->bits[i] ^ b->bits[i]));
    }

    return dist;
}
