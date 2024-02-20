#include <MMKV.h>
#include <string.h>

extern "C" void *getDefaultMMKV() {
    return MMKV::defaultMMKV();
}

extern "C" void initializeMMKV(const std::string &dir) {
    MMKV::initializeMMKV(dir);
}

extern "C" void set(MMKV *mmkv,float v,const std::string &k) {
    mmkv->set(v,k);
}

extern "C" float get_float(MMKV *mmkv,const std::string &k) {
    return mmkv->getFloat(k);
}