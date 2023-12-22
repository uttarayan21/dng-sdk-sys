/* Copyright (c) the JPEG XL Project Authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style
 * license that can be found in the LICENSE file.
 */

/** @addtogroup libjxl_common
 * @{
 * @file version.h
 * @brief libjxl version information
 */

#ifndef JXL_VERSION_H_
#define JXL_VERSION_H_

#define JPEGXL_MAJOR_VERSION 0
#define JPEGXL_MINOR_VERSION 8
#define JPEGXL_PATCH_VERSION 1

/** Can be used to conditionally compile code for a specific JXL version
 * @param[maj] major version
 * @param[min] minor version
 *
 * @code
 * #if JPEGXL_NUMERIC_VERSION < JPEGXL_COMPUTE_NUMERIC_VERSION(0,8,1)
 * // use old/deprecated api
 * #else
 * // use current api
 * #endif
 * @endcode
 */
#define JPEGXL_COMPUTE_NUMERIC_VERSION(major,minor,patch) ((major<<24) | (minor<<16) | (patch<<8) | 0)

/* Numeric representation of the version */
#define JPEGXL_NUMERIC_VERSION JPEGXL_COMPUTE_NUMERIC_VERSION(JPEGXL_MAJOR_VERSION,JPEGXL_MINOR_VERSION,JPEGXL_PATCH_VERSION)

#endif /* JXL_VERSION_H_ */

/** @}*/
