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

extern "C" void set_float(MMKV *mmkv,float v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" float get_float(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getFloat(tmp);
}
