#define RF_RENDERER_IMPL          //You must define this in at least one .c/.cpp files to include the implementation
#define RF_GRAPHICS_API_OPENGL_33 //Choose a graphics API
#include "glad.h"
#include "rayfork_renderer.h" //Include rayfork

typedef void *(*RFRSloadproc)(const char *name, void *userdata);

void *_rfrsLoadProcUserData;
RFRSloadproc _rfrsLoadProcProc;

void *_rfrsGLADLoadGLLoader(const char *name)
{
    return _rfrsLoadProcProc(name, _rfrsLoadProcUserData);
}

int rfrsLoadGL(RFRSloadproc proc, void *userdata)
{
    _rfrsLoadProcUserData = userdata;
    _rfrsLoadProcProc = proc;
    return gladLoadGLLoader(_rfrsGLADLoadGLLoader);
}
