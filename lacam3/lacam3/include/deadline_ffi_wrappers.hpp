#pragma once
#include <memory>

struct Deadline;

#ifdef __cplusplus
#endif

std::unique_ptr<Deadline> lacam_rs_create_deadline(double time_limit_ms);
bool lacam_rs_deadline_is_expired_ref(const Deadline& deadline);

#ifdef _cplusplus
#endif
