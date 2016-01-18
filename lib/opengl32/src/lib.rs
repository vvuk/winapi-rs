// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to opengl32.
#![cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn GlmfBeginGlsBlock();
    // pub fn GlmfCloseMetaFile();
    // pub fn GlmfEndGlsBlock();
    // pub fn GlmfEndPlayback();
    // pub fn GlmfInitPlayback();
    // pub fn GlmfPlayGlsRecord();
    pub fn glAccum(op: GLenum, value: GLfloat);
    pub fn glAlphaFunc(func: GLenum, reference: GLclampf);
    // pub fn glAreTexturesResident();
    // pub fn glArrayElement();
    // pub fn glBegin();
    // pub fn glBindTexture();
    // pub fn glBitmap();
    // pub fn glBlendFunc();
    // pub fn glCallList();
    // pub fn glCallLists();
    // pub fn glClear();
    // pub fn glClearAccum();
    // pub fn glClearColor();
    // pub fn glClearDepth();
    // pub fn glClearIndex();
    // pub fn glClearStencil();
    // pub fn glClipPlane();
    // pub fn glColor3b();
    // pub fn glColor3bv();
    // pub fn glColor3d();
    // pub fn glColor3dv();
    // pub fn glColor3f();
    // pub fn glColor3fv();
    // pub fn glColor3i();
    // pub fn glColor3iv();
    // pub fn glColor3s();
    // pub fn glColor3sv();
    // pub fn glColor3ub();
    // pub fn glColor3ubv();
    // pub fn glColor3ui();
    // pub fn glColor3uiv();
    // pub fn glColor3us();
    // pub fn glColor3usv();
    // pub fn glColor4b();
    // pub fn glColor4bv();
    // pub fn glColor4d();
    // pub fn glColor4dv();
    // pub fn glColor4f();
    // pub fn glColor4fv();
    // pub fn glColor4i();
    // pub fn glColor4iv();
    // pub fn glColor4s();
    // pub fn glColor4sv();
    // pub fn glColor4ub();
    // pub fn glColor4ubv();
    // pub fn glColor4ui();
    // pub fn glColor4uiv();
    // pub fn glColor4us();
    // pub fn glColor4usv();
    // pub fn glColorMask();
    // pub fn glColorMaterial();
    // pub fn glColorPointer();
    // pub fn glCopyPixels();
    // pub fn glCopyTexImage1D();
    // pub fn glCopyTexImage2D();
    // pub fn glCopyTexSubImage1D();
    // pub fn glCopyTexSubImage2D();
    // pub fn glCullFace();
    // pub fn glDebugEntry();
    // pub fn glDeleteLists();
    // pub fn glDeleteTextures();
    // pub fn glDepthFunc();
    // pub fn glDepthMask();
    // pub fn glDepthRange();
    // pub fn glDisable();
    // pub fn glDisableClientState();
    // pub fn glDrawArrays();
    // pub fn glDrawBuffer();
    // pub fn glDrawElements();
    // pub fn glDrawPixels();
    // pub fn glEdgeFlag();
    // pub fn glEdgeFlagPointer();
    // pub fn glEdgeFlagv();
    // pub fn glEnable();
    // pub fn glEnableClientState();
    // pub fn glEnd();
    // pub fn glEndList();
    // pub fn glEvalCoord1d();
    // pub fn glEvalCoord1dv();
    // pub fn glEvalCoord1f();
    // pub fn glEvalCoord1fv();
    // pub fn glEvalCoord2d();
    // pub fn glEvalCoord2dv();
    // pub fn glEvalCoord2f();
    // pub fn glEvalCoord2fv();
    // pub fn glEvalMesh1();
    // pub fn glEvalMesh2();
    // pub fn glEvalPoint1();
    // pub fn glEvalPoint2();
    // pub fn glFeedbackBuffer();
    // pub fn glFinish();
    // pub fn glFlush();
    // pub fn glFogf();
    // pub fn glFogfv();
    // pub fn glFogi();
    // pub fn glFogiv();
    // pub fn glFrontFace();
    // pub fn glFrustum();
    // pub fn glGenLists();
    // pub fn glGenTextures();
    // pub fn glGetBooleanv();
    // pub fn glGetClipPlane();
    // pub fn glGetDoublev();
    // pub fn glGetError();
    // pub fn glGetFloatv();
    // pub fn glGetIntegerv();
    // pub fn glGetLightfv();
    // pub fn glGetLightiv();
    // pub fn glGetMapdv();
    // pub fn glGetMapfv();
    // pub fn glGetMapiv();
    // pub fn glGetMaterialfv();
    // pub fn glGetMaterialiv();
    // pub fn glGetPixelMapfv();
    // pub fn glGetPixelMapuiv();
    // pub fn glGetPixelMapusv();
    // pub fn glGetPointerv();
    // pub fn glGetPolygonStipple();
    // pub fn glGetString();
    // pub fn glGetTexEnvfv();
    // pub fn glGetTexEnviv();
    // pub fn glGetTexGendv();
    // pub fn glGetTexGenfv();
    // pub fn glGetTexGeniv();
    // pub fn glGetTexImage();
    // pub fn glGetTexLevelParameterfv();
    // pub fn glGetTexLevelParameteriv();
    // pub fn glGetTexParameterfv();
    // pub fn glGetTexParameteriv();
    // pub fn glHint();
    // pub fn glIndexMask();
    // pub fn glIndexPointer();
    // pub fn glIndexd();
    // pub fn glIndexdv();
    // pub fn glIndexf();
    // pub fn glIndexfv();
    // pub fn glIndexi();
    // pub fn glIndexiv();
    // pub fn glIndexs();
    // pub fn glIndexsv();
    // pub fn glIndexub();
    // pub fn glIndexubv();
    // pub fn glInitNames();
    // pub fn glInterleavedArrays();
    // pub fn glIsEnabled();
    // pub fn glIsList();
    // pub fn glIsTexture();
    // pub fn glLightModelf();
    // pub fn glLightModelfv();
    // pub fn glLightModeli();
    // pub fn glLightModeliv();
    // pub fn glLightf();
    // pub fn glLightfv();
    // pub fn glLighti();
    // pub fn glLightiv();
    // pub fn glLineStipple();
    // pub fn glLineWidth();
    // pub fn glListBase();
    // pub fn glLoadIdentity();
    // pub fn glLoadMatrixd();
    // pub fn glLoadMatrixf();
    // pub fn glLoadName();
    // pub fn glLogicOp();
    // pub fn glMap1d();
    // pub fn glMap1f();
    // pub fn glMap2d();
    // pub fn glMap2f();
    // pub fn glMapGrid1d();
    // pub fn glMapGrid1f();
    // pub fn glMapGrid2d();
    // pub fn glMapGrid2f();
    // pub fn glMaterialf();
    // pub fn glMaterialfv();
    // pub fn glMateriali();
    // pub fn glMaterialiv();
    // pub fn glMatrixMode();
    // pub fn glMultMatrixd();
    // pub fn glMultMatrixf();
    // pub fn glNewList();
    // pub fn glNormal3b();
    // pub fn glNormal3bv();
    // pub fn glNormal3d();
    // pub fn glNormal3dv();
    // pub fn glNormal3f();
    // pub fn glNormal3fv();
    // pub fn glNormal3i();
    // pub fn glNormal3iv();
    // pub fn glNormal3s();
    // pub fn glNormal3sv();
    // pub fn glNormalPointer();
    // pub fn glOrtho();
    // pub fn glPassThrough();
    // pub fn glPixelMapfv();
    // pub fn glPixelMapuiv();
    // pub fn glPixelMapusv();
    // pub fn glPixelStoref();
    // pub fn glPixelStorei();
    // pub fn glPixelTransferf();
    // pub fn glPixelTransferi();
    // pub fn glPixelZoom();
    // pub fn glPointSize();
    // pub fn glPolygonMode();
    // pub fn glPolygonOffset();
    // pub fn glPolygonStipple();
    // pub fn glPopAttrib();
    // pub fn glPopClientAttrib();
    // pub fn glPopMatrix();
    // pub fn glPopName();
    // pub fn glPrioritizeTextures();
    // pub fn glPushAttrib();
    // pub fn glPushClientAttrib();
    // pub fn glPushMatrix();
    // pub fn glPushName();
    // pub fn glRasterPos2d();
    // pub fn glRasterPos2dv();
    // pub fn glRasterPos2f();
    // pub fn glRasterPos2fv();
    // pub fn glRasterPos2i();
    // pub fn glRasterPos2iv();
    // pub fn glRasterPos2s();
    // pub fn glRasterPos2sv();
    // pub fn glRasterPos3d();
    // pub fn glRasterPos3dv();
    // pub fn glRasterPos3f();
    // pub fn glRasterPos3fv();
    // pub fn glRasterPos3i();
    // pub fn glRasterPos3iv();
    // pub fn glRasterPos3s();
    // pub fn glRasterPos3sv();
    // pub fn glRasterPos4d();
    // pub fn glRasterPos4dv();
    // pub fn glRasterPos4f();
    // pub fn glRasterPos4fv();
    // pub fn glRasterPos4i();
    // pub fn glRasterPos4iv();
    // pub fn glRasterPos4s();
    // pub fn glRasterPos4sv();
    // pub fn glReadBuffer();
    // pub fn glReadPixels();
    // pub fn glRectd();
    // pub fn glRectdv();
    // pub fn glRectf();
    // pub fn glRectfv();
    // pub fn glRecti();
    // pub fn glRectiv();
    // pub fn glRects();
    // pub fn glRectsv();
    // pub fn glRenderMode();
    // pub fn glRotated();
    // pub fn glRotatef();
    // pub fn glScaled();
    // pub fn glScalef();
    // pub fn glScissor();
    // pub fn glSelectBuffer();
    // pub fn glShadeModel();
    // pub fn glStencilFunc();
    // pub fn glStencilMask();
    // pub fn glStencilOp();
    // pub fn glTexCoord1d();
    // pub fn glTexCoord1dv();
    // pub fn glTexCoord1f();
    // pub fn glTexCoord1fv();
    // pub fn glTexCoord1i();
    // pub fn glTexCoord1iv();
    // pub fn glTexCoord1s();
    // pub fn glTexCoord1sv();
    // pub fn glTexCoord2d();
    // pub fn glTexCoord2dv();
    // pub fn glTexCoord2f();
    // pub fn glTexCoord2fv();
    // pub fn glTexCoord2i();
    // pub fn glTexCoord2iv();
    // pub fn glTexCoord2s();
    // pub fn glTexCoord2sv();
    // pub fn glTexCoord3d();
    // pub fn glTexCoord3dv();
    // pub fn glTexCoord3f();
    // pub fn glTexCoord3fv();
    // pub fn glTexCoord3i();
    // pub fn glTexCoord3iv();
    // pub fn glTexCoord3s();
    // pub fn glTexCoord3sv();
    // pub fn glTexCoord4d();
    // pub fn glTexCoord4dv();
    // pub fn glTexCoord4f();
    // pub fn glTexCoord4fv();
    // pub fn glTexCoord4i();
    // pub fn glTexCoord4iv();
    // pub fn glTexCoord4s();
    // pub fn glTexCoord4sv();
    // pub fn glTexCoordPointer();
    // pub fn glTexEnvf();
    // pub fn glTexEnvfv();
    // pub fn glTexEnvi();
    // pub fn glTexEnviv();
    // pub fn glTexGend();
    // pub fn glTexGendv();
    // pub fn glTexGenf();
    // pub fn glTexGenfv();
    // pub fn glTexGeni();
    // pub fn glTexGeniv();
    // pub fn glTexImage1D();
    // pub fn glTexImage2D();
    // pub fn glTexParameterf();
    // pub fn glTexParameterfv();
    // pub fn glTexParameteri();
    // pub fn glTexParameteriv();
    // pub fn glTexSubImage1D();
    // pub fn glTexSubImage2D();
    // pub fn glTranslated();
    // pub fn glTranslatef();
    // pub fn glVertex2d();
    // pub fn glVertex2dv();
    // pub fn glVertex2f();
    // pub fn glVertex2fv();
    // pub fn glVertex2i();
    // pub fn glVertex2iv();
    // pub fn glVertex2s();
    // pub fn glVertex2sv();
    // pub fn glVertex3d();
    // pub fn glVertex3dv();
    // pub fn glVertex3f();
    // pub fn glVertex3fv();
    // pub fn glVertex3i();
    // pub fn glVertex3iv();
    // pub fn glVertex3s();
    // pub fn glVertex3sv();
    // pub fn glVertex4d();
    // pub fn glVertex4dv();
    // pub fn glVertex4f();
    // pub fn glVertex4fv();
    // pub fn glVertex4i();
    // pub fn glVertex4iv();
    // pub fn glVertex4s();
    // pub fn glVertex4sv();
    // pub fn glVertexPointer();
    // pub fn glViewport();
    // pub fn wglChoosePixelFormat();
    pub fn wglCopyContext(hglrcSrc: HGLRC, hglrcDst: HGLRC, mask: UINT) -> BOOL;
    pub fn wglCreateContext(hdc: HDC) -> HGLRC;
    pub fn wglCreateLayerContext(hdc: HDC, iLayerPlane: c_int) -> HGLRC;
    pub fn wglDeleteContext(hglrc: HGLRC) -> BOOL;
    pub fn wglDescribeLayerPlane(
        hdc: HDC, iPixelFormat: c_int, iLayerPlane: c_int, nBytes: UINT,
        plpd: LPLAYERPLANEDESCRIPTOR
    ) -> BOOL;
    // pub fn wglDescribePixelFormat();
    pub fn wglGetCurrentContext() -> HGLRC;
    pub fn wglGetCurrentDC() -> HDC;
    // pub fn wglGetDefaultProcAddress();
    pub fn wglGetLayerPaletteEntries(
        hdc: HDC, iLayerPlane: c_int, iStart: c_int, cEntries: c_int, pcr: *const COLORREF
    ) -> c_int;
    // pub fn wglGetPixelFormat();
    pub fn wglGetProcAddress(lpszProc: LPCSTR) -> PROC;
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;
    pub fn wglRealizeLayerPalette(hdc: HDC, iLayerPlane: c_int, bRealize: BOOL) -> BOOL;
    pub fn wglSetLayerPaletteEntries(
        hdc: HDC, iLayerPlane: c_int, iStart: c_int, cEntries: c_int, pcr: *const COLORREF
    ) -> c_int;
    // pub fn wglSetPixelFormat();
    pub fn wglShareLists(hglrc1: HGLRC, hglrc2: HGLRC) -> BOOL;
    // pub fn wglSwapBuffers();
    pub fn wglSwapLayerBuffers(hdc: HDC, fuPlanes: UINT) -> BOOL;
    // pub fn wglSwapMultipleBuffers();
    pub fn wglUseFontBitmapsA(hdc: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL;
    pub fn wglUseFontBitmapsW(hdc: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL;
    pub fn wglUseFontOutlinesA(
        hdc: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT,
        extrusion: FLOAT, format: c_int, lpgmf: LPGLYPHMETRICSFLOAT
    ) -> BOOL;
    pub fn wglUseFontOutlinesW(
        hdc: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT,
        extrusion: FLOAT, format: c_int, lpgmf: LPGLYPHMETRICSFLOAT
    ) -> BOOL;
}