#ifndef WRAPPER_H_
#define WRAPPER_H_

#include <dds/dds.h>
#include <dds/ddsc/dds_public_qosdefs.h>
#include <dds/ddsi/ddsi_radmin.h>
#include <dds/ddsi/ddsi_serdata.h>
#include <dds/ddsi/ddsi_sertype.h>

// Wrapper Functions:
//
// Inline functions can't be processed in a sensible fashion by `bindgen`. Such
// functions that need to be accessed are wrapped here with the
// `_bindgen_wrapper` suffix. The `build.rs` strips these suffixes out when
// generating the binding and remaps the underlying symbol.  As a result, these
// functions can be accessed on Rust side without the `_bindgen_wrapper` suffix.
//
// If the function they wrap is not inline and the wrapper has the same name
// this will lead to a failure in compilation as duplicate symbols will be
// present at link time. This is desirable as the wrappers may be added safely
// for the inline functions and if they change downstream to no longer be inline
// the wrapper function may simply be removed.

/// Increment a serdata.
static struct ddsi_serdata *ddsi_serdata_ref_bindgen_wrapper(
    const struct ddsi_serdata *serdata_const) {
  return ddsi_serdata_ref(serdata_const);
}

/// Decrement a serdata.
static void ddsi_serdata_unref_bindgen_wrapper(struct ddsi_serdata *serdata) {
  ddsi_serdata_unref(serdata);
}

static uint32_t ddsrt_atomic_ld32_bindgen_wrapper(
    const volatile ddsrt_atomic_uint32_t *x) {
  return ddsrt_atomic_ld32(x);
}

static void ddsrt_atomic_st32_bindgen_wrapper(volatile ddsrt_atomic_uint32_t *x,
                                              uint32_t v) {
  ddsrt_atomic_st32(x, v);
}

static const uint32_t DOMAIN_DEFAULT = DDS_DOMAIN_DEFAULT;
static const dds_duration_t DURATION_INFINITE = DDS_INFINITY;
static const dds_time_t TIME_NEVER = DDS_NEVER;
static const dds_entity_t BUILTIN_TOPIC_DCPS_PARTICIPANT =
    DDS_BUILTIN_TOPIC_DCPSPARTICIPANT;
static const dds_entity_t BUILTIN_TOPIC_DCPS_TOPIC =
    DDS_BUILTIN_TOPIC_DCPSTOPIC;
static const dds_entity_t BUILTIN_TOPIC_DCPS_PUBLICATION =
    DDS_BUILTIN_TOPIC_DCPSPUBLICATION;
static const dds_entity_t BUILTIN_TOPIC_DCPS_SUBSCRIPTION =
    DDS_BUILTIN_TOPIC_DCPSSUBSCRIPTION;

#endif  // WRAPPER_H_
