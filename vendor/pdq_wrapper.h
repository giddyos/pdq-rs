#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct
    {
        uint8_t bits[32];
    } PdqHash256;

    int pdq_hash_from_rgb(const uint8_t *rgb, int num_rows, int num_cols, PdqHash256 *out_hash);

    int pdq_hash_from_grey(const uint8_t *grey, int num_rows, int num_cols, PdqHash256 *out_hash);

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
        PdqHash256 *out_flip_minus1);

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
        PdqHash256 *out_flip_minus1);

    int pdq_hamming_distance(const PdqHash256 *a, const PdqHash256 *b);

#ifdef __cplusplus
}
#endif
