#include "../include/deadline_ffi_wrappers.hpp"
#include "../include/utils.hpp"
#include <memory>

std::unique_ptr<Deadline> lacam_rs_create_deadline(double time_limit_ms) {
    try {
        return std::make_unique<Deadline>(time_limit_ms);
    } catch(...) {
        return nullptr;
    }
}

bool lacam_rs_deadline_is_expired_ref(const Deadline& deadline) {
    return::is_expired(&deadline);
}