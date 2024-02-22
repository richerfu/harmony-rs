#include <MMKV.h>
#include <string>

// C ABI
extern "C" MMKV *get_mmkv_instance() {
    return MMKV::defaultMMKV();
}

extern "C" void init_mmkv() {
//    std::string tmp(dir);
    std::string tmp = "/data/storage/el2/base/haps/entry/files/mmkv";
    MMKV::initializeMMKV(tmp);
}

extern "C" void set_float(MMKV *mmkv,int32_t v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" int32_t get_float(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getInt32(tmp);
}
