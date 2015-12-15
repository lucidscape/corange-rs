#include "SDL2/SDL_local.h"
#include "SDL2/SDL_rwops.h"

#include <stdlib.h>
#include <stdio.h>
#include <dirent.h>
#include <unistd.h>

#ifdef _WIN32
  #include <windows.h>
  #include <winbase.h>
#endif

#ifdef __unix__
  #include <execinfo.h>
#endif

#ifdef _WIN32

void SDL_PathFullName(char* dst, const char* path) {
  GetFullPathName(path, MAX_PATH, dst, NULL);
}

#elif __unix__

void SDL_PathFullName(char* dst, const char* path) {
  char* ret = realpath(path, dst);
}

#endif

void SDL_PathFileName(char* dst, const char* path) {
  
  int i = strlen(path);
  int ext_loc = 0;
  while( i > 0) {
    if (path[i] == '/') { break; }
    if (path[i] == '\\') { break; }
    if (path[i] == '.') { ext_loc = i; }
    i--;
  }
  
  const char* file = path + i + 1;
  int len = ext_loc - i - 1;
  
  strncpy(dst, file, len);
  dst[len] = '\0';
}

void SDL_PathFileExtension(char* dst, const char* path) {

  int ext_len = 0;
  int i = strlen(path);
  while( i >= 0) {
    if (path[i] != '.') { ext_len++; }
    if (path[i] == '.') { break; }
    i--;
  }
  
  int prev = strlen(path) - ext_len + 1;
  const char* f_ext = path + prev;
  strcpy(dst, f_ext);
}

void SDL_PathFileLocation(char* dst, const char* path) {
  
  int i = strlen(path);
  while( i > 0) {
    if (path[i] == '/') { break; }
    if (path[i] == '\\') { break; }
    i--;
  }
  i++;
  
  strncpy(dst, path, i);
  dst[i] = '\0';
}

void SDL_PathParentDirectory(char* dst, const char* path) {

  int i = strlen(path)-1;
  while( i > 0) {
    if (path[i] == '/') { break; }
    if (path[i] == '\\') { break; }
    i--;
  }
  
  strncpy(dst, path, i);
  dst[i] = '\0';
  
}

void SDL_PathRelative(char* dst, const char* path) {
  char* curr = SDL_GetWorkingDir();
  char* sub = strstr(path, curr);
  
  if (sub == NULL) {
    strcpy(dst, path);
  } else {
    strcpy(dst, ".");
    strcat(dst, path + strlen(curr));
  }
  
}

void SDL_PathForwardSlashes(char* path) {
  for(int i = 0; i < strlen(path); i++) {
    if (path[i] == '\\') { path[i] = '/'; }
  }
}

void SDL_PathJoin(char* dst, char* fst, char* snd) {
  
}

bool SDL_PathIsFile(char* path) {
  SDL_RWops* f = SDL_RWFromFile(path, "r");
  if (f == NULL) { return false; }
  else { SDL_RWclose(f); return true; }
}

bool SDL_PathIsDirectory(char* path) {
  DIR* d = opendir(path);
  if (d == NULL) { return false; }
  else { closedir(d); return true; }
}

static char curr_dir[MAX_PATH];

char* SDL_GetWorkingDir() {
  char* discard = getcwd(curr_dir, sizeof(curr_dir));
  return curr_dir;
}

int SDL_SetWorkingDir(char* dir) {
  int err = chdir(dir);
  return err;
}

void SDL_GL_PrintInfo() {
  const char* vendor = (const char*)glGetString(GL_VENDOR);
  const char* renderer = (const char*)glGetString(GL_RENDERER);
  const char* version = (const char*)glGetString(GL_VERSION);
  const char* shader_version = (const char*)glGetString(GL_SHADING_LANGUAGE_VERSION);
  
  printf("OpenGL Info\n");
  printf("Vendor: %s\n", vendor);
  printf("Renderer: %s\n", renderer);
  printf("Version: %s\n", version);
  printf("Shader Version: %s\n", shader_version);
}

void SDL_GL_PrintExtensions() {
  const char* extensions = (const char*)glGetString(GL_EXTENSIONS);
  printf("OpenGL Extensions: %s\n", extensions);
}

static const char* gl_error_string_invalid_enum = "Invalid Enum";
static const char* gl_error_string_invalid_value = "Invalid Value";
static const char* gl_error_string_invalid_operation = "Invalid Operation";
static const char* gl_error_string_out_of_memory = "Out of Memory";
static const char* gl_error_string_invalid_framebuffer_operation = "Invalid Framebuffer Operation";
static const char* gl_error_string_stack_overflow = "Stack Overflow";
static const char* gl_error_string_stack_underflow = "Stack Underflow";
static const char* gl_error_string_table_too_large = "Table Too Large";
static const char* gl_error_string_no_error = "No Error";

const char* SDL_GL_ErrorString(GLenum error) {
  switch (error) {
    case GL_INVALID_ENUM:
      return gl_error_string_invalid_enum;
    case GL_INVALID_VALUE:
      return gl_error_string_invalid_value;
    case GL_INVALID_OPERATION:
      return gl_error_string_invalid_operation;
    case GL_OUT_OF_MEMORY:
      return gl_error_string_out_of_memory;
    case GL_INVALID_FRAMEBUFFER_OPERATION:
      return gl_error_string_invalid_framebuffer_operation;
    case GL_STACK_OVERFLOW:
      return gl_error_string_stack_overflow;
    case GL_STACK_UNDERFLOW:
      return gl_error_string_stack_underflow;
    case GL_TABLE_TOO_LARGE:
      return gl_error_string_table_too_large;
  }
  return gl_error_string_no_error;
}

static const char* gl_error_string_framebuffer_complete = "Framebuffer Complete";
static const char* gl_error_string_framebuffer_undefined = "Framebuffer Undefined";
static const char* gl_error_string_framebuffer_incomplete_attach = "Framebuffer Incomplete Attachment";
static const char* gl_error_string_framebuffer_incomplete_missing_attach = "Framebuffer No Attachments";
static const char* gl_error_string_framebuffer_incomplete_draw = "Framebuffer Incomplete Draw";
static const char* gl_error_string_framebuffer_incomplete_read = "Framebuffer Incomplete Read";
static const char* gl_error_string_framebuffer_unsupported = "Framebuffer Unsupported";
static const char* gl_error_string_framebuffer_incomplete_multisample = "Framebuffer Badly Configured Multisamples";
static const char* gl_error_string_framebuffer_incomplete_layer_targets = "Framebuffer Badly Configured Layer Targets";

const char* SDL_GL_FrameBufferErrorString(GLenum error) {
  switch(error) {
    case GL_FRAMEBUFFER_COMPLETE:
      return gl_error_string_framebuffer_complete;
    case GL_FRAMEBUFFER_UNDEFINED:
      return gl_error_string_framebuffer_undefined;
    case GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT:
      return gl_error_string_framebuffer_incomplete_attach;
    case GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT:
      return gl_error_string_framebuffer_incomplete_missing_attach;
    case GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER:
      return gl_error_string_framebuffer_incomplete_draw;
    case GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER:
      return gl_error_string_framebuffer_incomplete_read;
    case GL_FRAMEBUFFER_UNSUPPORTED:
      return gl_error_string_framebuffer_unsupported;
    case GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE:
      return gl_error_string_framebuffer_incomplete_multisample;
    case GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS:
      return gl_error_string_framebuffer_incomplete_layer_targets;
  }
  return gl_error_string_no_error;
}

bool SDL_GL_ExtensionPresent(char* name) {
  const char* extensions = (const char*)glGetString(GL_EXTENSIONS);
  if (strstr(extensions, name)) {
    return true;
  } else {
    return false;
  }
}

bool SDL_GL_ExtensionFuncionLoaded(void* function) {
  if (function == NULL) {
    return false;
  } else {
    return true;
  }
}

#ifndef __unix__
GLACTIVETEXTUREFN glActiveTexture = NULL;
GLCOMPRESSEDTEXIMAGE2DFN glCompressedTexImage2D = NULL;
GLTEXIMAGE3DFN glTexImage3D = NULL;
#endif
GLCREATESHADERFN glCreateShader = NULL;
GLCREATEPROGRAMFN glCreateProgram = NULL;
GLSHADERSOURCEFN glShaderSource = NULL;
GLCOMPILESHADERFN glCompileShader = NULL;
GLGETSHADERINFOLOGFN glGetShaderInfoLog = NULL;
GLATTACHSHADERFN glAttachShader = NULL;
GLLINKPROGRAMFN glLinkProgram = NULL;
GLGETPROGRAMINFOLOGFN glGetProgramInfoLog = NULL;
GLISPROGRAMFN glIsProgram = NULL;
GLISSHADERFN glIsShader = NULL;
GLGETATTACHEDSHADERSFN glGetAttachedShaders = NULL;
GLGETUNIFORMLOCATIONFN glGetUniformLocation = NULL;
GLUNIFORM1FFN glUniform1f = NULL;
GLUNIFORM1IFN glUniform1i = NULL;
GLDELETESHADERFN glDeleteShader = NULL;
GLDELETEPROGRAMFN glDeleteProgram = NULL;
GLUSEPROGRAMFN glUseProgram = NULL;
GLVERTEXATTRIBPOINTERFN glVertexAttribPointer = NULL;
GLVERTEXATTRIBDIVISORFN glVertexAttribDivisor = NULL;
GLENABLEVERTEXATTRIBARRAYFN glEnableVertexAttribArray = NULL;
GLDISABLEVERTEXATTRIBARRAYFN glDisableVertexAttribArray = NULL;
GLUNIFORM2FFN glUniform2f = NULL;
GLUNIFORM3FFN glUniform3f = NULL;
GLUNIFORM4FFN glUniform4f = NULL;
GLUNIFORMMATRIX3FVFN glUniformMatrix3fv = NULL;
GLUNIFORMMATRIX4FVFN glUniformMatrix4fv = NULL;
GLUNIFORM1FVFN glUniform1fv = NULL;
GLUNIFORM2FVFN glUniform2fv = NULL;
GLUNIFORM3FVFN glUniform3fv = NULL;
GLUNIFORM3FVFN glUniform4fv = NULL;
GLGETSHADERIVFN glGetShaderiv = NULL;
GLPROGRAMPARAMETERIFN glProgramParameteri = NULL;
GLGETPROGRAMIVFN glGetProgramiv = NULL;
GLBINDATTRIBLOCATIONFN glBindAttribLocation = NULL;
GLGENFRAMEBUFFERSFN glGenFramebuffers = NULL;
GLBINDFRAMEBUFFERFN glBindFramebuffer = NULL;
GLBLITFRAMEBUFFERFN glBlitFramebuffer = NULL;
GLFRAMEBUFFERTEXTUREFN glFramebufferTexture = NULL;
GLFRAMEBUFFERTEXTURE2DFN glFramebufferTexture2D = NULL;
GLDELETEFRAMEBUFFERSFN glDeleteFramebuffers = NULL;
GLCHECKFRAMEBUFFERSTATUSFN glCheckFramebufferStatus = NULL;
GLGENBUFFERSFN glGenBuffers = NULL;
GLGENRENDERBUFFERSFN glGenRenderbuffers = NULL;
GLDELETEBUFFERSFN glDeleteBuffers = NULL;
GLDELETERENDERBUFFERSFN glDeleteRenderbuffers = NULL;
GLBINDBUFFERFN glBindBuffer = NULL;
GLBINDRENDERBUFFERFN glBindRenderbuffer = NULL;
GLBUFFERDATAFN glBufferData = NULL;
GLGETBUFFERSUBDATAFN glGetBufferSubData = NULL;
GLFRAMEBUFFERRENDERBUFFERFN glFramebufferRenderbuffer = NULL;
GLGETATTRIBLOCATIONFN glGetAttribLocation = NULL;
GLRENDERBUFFERSTORAGEFN glRenderbufferStorage = NULL;
GLRENDERBUFFERSTORAGEMULTISAMPLEFN glRenderbufferStorageMultisample = NULL;
GLDRAWBUFFERSFN glDrawBuffers = NULL;
GLGENERATEMIPMAPFN glGenerateMipmap = NULL;
GLDRAWELEMENTSINSTANCEDFN glDrawElementsInstanced = NULL;
GLPATCHPARAMETERIFN glPatchParameteri = NULL;
GLPATCHPARAMETERFVFN glPatchParameterfv = NULL;

GLBROKENEXTENSIONFN glBrokenExtension = NULL;

#define SDL_GL_LoadExtension(type, name) \
name = (type)SDL_GL_GetProcAddress(#name); \
if (name == NULL) { \
  fprintf(stderr, "Failed to load function '%s', looking for function '%s'...\n", #name, #name"EXT"); \
  name = (type)SDL_GL_GetProcAddress(#name"EXT"); \
} \
if (name == NULL) { \
  fprintf(stderr, "Failed to load function '%s', looking for function '%s'...\n", #name"EXT", #name"ARB"); \
  name = (type)SDL_GL_GetProcAddress(#name"ARB"); \
} \
if (name == NULL) { fprintf(stderr, "Completely failed to load OpenGL extension function '%s'. Use of this function will crash\n", #name); }
  
void SDL_GL_LoadExtensions() {

  /* Shaders */
  
  SDL_GL_LoadExtension(GLCREATEPROGRAMFN, glCreateProgram);
  SDL_GL_LoadExtension(GLLINKPROGRAMFN, glLinkProgram);
  SDL_GL_LoadExtension(GLDELETEPROGRAMFN, glDeleteProgram);
  SDL_GL_LoadExtension(GLGETPROGRAMINFOLOGFN, glGetProgramInfoLog);
  SDL_GL_LoadExtension(GLUSEPROGRAMFN, glUseProgram);
  SDL_GL_LoadExtension(GLGETPROGRAMIVFN, glGetProgramiv);
  SDL_GL_LoadExtension(GLPROGRAMPARAMETERIFN, glProgramParameteri);
  
  SDL_GL_LoadExtension(GLCREATESHADERFN, glCreateShader);
  SDL_GL_LoadExtension(GLSHADERSOURCEFN, glShaderSource);
  SDL_GL_LoadExtension(GLCOMPILESHADERFN, glCompileShader);
  SDL_GL_LoadExtension(GLGETSHADERINFOLOGFN, glGetShaderInfoLog);
  SDL_GL_LoadExtension(GLATTACHSHADERFN, glAttachShader);
  SDL_GL_LoadExtension(GLDELETESHADERFN, glDeleteShader);
  SDL_GL_LoadExtension(GLGETSHADERIVFN, glGetShaderiv);
  SDL_GL_LoadExtension(GLISPROGRAMFN, glIsProgram);
  SDL_GL_LoadExtension(GLISSHADERFN, glIsShader);
  SDL_GL_LoadExtension(GLGETATTACHEDSHADERSFN, glGetAttachedShaders);
  
  SDL_GL_LoadExtension(GLGETUNIFORMLOCATIONFN, glGetUniformLocation);
  SDL_GL_LoadExtension(GLUNIFORM1FFN, glUniform1f);
  SDL_GL_LoadExtension(GLUNIFORM1IFN, glUniform1i);
  SDL_GL_LoadExtension(GLUNIFORM2FFN, glUniform2f);
  SDL_GL_LoadExtension(GLUNIFORM3FFN, glUniform3f);
  SDL_GL_LoadExtension(GLUNIFORM4FFN, glUniform4f);
  SDL_GL_LoadExtension(GLUNIFORM1FVFN, glUniform1fv);
  SDL_GL_LoadExtension(GLUNIFORM2FVFN, glUniform2fv);
  SDL_GL_LoadExtension(GLUNIFORM3FVFN, glUniform3fv);
  SDL_GL_LoadExtension(GLUNIFORM3FVFN, glUniform4fv);
  SDL_GL_LoadExtension(GLUNIFORMMATRIX3FVFN, glUniformMatrix3fv);
  SDL_GL_LoadExtension(GLUNIFORMMATRIX4FVFN, glUniformMatrix4fv);
  
  /* Attributes */
  
  SDL_GL_LoadExtension(GLGETATTRIBLOCATIONFN, glGetAttribLocation);
  SDL_GL_LoadExtension(GLVERTEXATTRIBPOINTERFN, glVertexAttribPointer);
  SDL_GL_LoadExtension(GLVERTEXATTRIBDIVISORFN, glVertexAttribDivisor);
  SDL_GL_LoadExtension(GLENABLEVERTEXATTRIBARRAYFN, glEnableVertexAttribArray);
  SDL_GL_LoadExtension(GLDISABLEVERTEXATTRIBARRAYFN, glDisableVertexAttribArray);
  SDL_GL_LoadExtension(GLBINDATTRIBLOCATIONFN, glBindAttribLocation);
  
  /* Textures */
  
  SDL_GL_LoadExtension(GLGENERATEMIPMAPFN, glGenerateMipmap);
  #ifndef __unix__
  SDL_GL_LoadExtension(GLACTIVETEXTUREFN, glActiveTexture);
  SDL_GL_LoadExtension(GLCOMPRESSEDTEXIMAGE2DFN, glCompressedTexImage2D);
  SDL_GL_LoadExtension(GLTEXIMAGE3DFN, glTexImage3D);
  #endif
  
  /* Buffers */
  
  SDL_GL_LoadExtension(GLGENBUFFERSFN, glGenBuffers);
  SDL_GL_LoadExtension(GLBINDBUFFERFN, glBindBuffer);
  SDL_GL_LoadExtension(GLBUFFERDATAFN, glBufferData);
  SDL_GL_LoadExtension(GLGETBUFFERSUBDATAFN, glGetBufferSubData);
  SDL_GL_LoadExtension(GLDELETEBUFFERSFN, glDeleteBuffers);
  SDL_GL_LoadExtension(GLDRAWBUFFERSFN, glDrawBuffers);
  
  SDL_GL_LoadExtension(GLGENRENDERBUFFERSFN, glGenRenderbuffers);
  SDL_GL_LoadExtension(GLBINDRENDERBUFFERFN, glBindRenderbuffer);
  SDL_GL_LoadExtension(GLRENDERBUFFERSTORAGEFN, glRenderbufferStorage);
  SDL_GL_LoadExtension(GLRENDERBUFFERSTORAGEMULTISAMPLEFN, glRenderbufferStorageMultisample);
  SDL_GL_LoadExtension(GLDELETERENDERBUFFERSFN, glDeleteRenderbuffers);
  
  SDL_GL_LoadExtension(GLGENFRAMEBUFFERSFN, glGenFramebuffers);
  SDL_GL_LoadExtension(GLBINDFRAMEBUFFERFN, glBindFramebuffer);
  SDL_GL_LoadExtension(GLBLITFRAMEBUFFERFN, glBlitFramebuffer);
  SDL_GL_LoadExtension(GLFRAMEBUFFERTEXTUREFN, glFramebufferTexture);
  SDL_GL_LoadExtension(GLFRAMEBUFFERTEXTURE2DFN, glFramebufferTexture2D);
  SDL_GL_LoadExtension(GLDELETEFRAMEBUFFERSFN, glDeleteFramebuffers);
  SDL_GL_LoadExtension(GLCHECKFRAMEBUFFERSTATUSFN, glCheckFramebufferStatus);
  SDL_GL_LoadExtension(GLFRAMEBUFFERRENDERBUFFERFN, glFramebufferRenderbuffer);
  
  /* Tessellation */
  
  SDL_GL_LoadExtension(GLPATCHPARAMETERIFN, glPatchParameteri);
  SDL_GL_LoadExtension(GLPATCHPARAMETERFVFN, glPatchParameterfv);
  
  /* Misc */
  
  SDL_GL_LoadExtension(GLDRAWELEMENTSINSTANCEDFN, glDrawElementsInstanced);
  
  /* Test for missing Extension */
  
  //SDL_GL_LoadExtension(GLBROKENEXTENSIONFN, glBrokenExtension);
  
}

#ifdef __unix__

void SDL_PrintStackTrace() {
}

#elif _WIN32

#include "DbgHelp.h"

void SDL_PrintStackTrace() {

  /*
	HANDLE process = GetCurrentProcess();
	HANDLE thread = GetCurrentThread();
  CONTEXT context;
	BOOL walking = TRUE;
	STACKFRAME64 stackframe;

  context.ContextFlags = CONTEXT_CONTROL;
  GetThreadContext(thread, &context);
  
	ZeroMemory(&stackframe, sizeof(STACKFRAME64));
	stackframe.AddrPC.Offset = context.Eip;
	stackframe.AddrPC.Mode = AddrModeFlat;
	stackframe.AddrFrame.Offset = context.Ebp;
	stackframe.AddrFrame.Mode = AddrModeFlat;
	stackframe.AddrStack.Offset = context.Esp;
	stackframe.AddrStack.Mode = AddrModeFlat;

	SymInitialize(process, NULL, TRUE);

	while (walking) {
		walking = StackWalk64(
      IMAGE_FILE_MACHINE_I386, process, thread, 
      &stackframe, &context, 
      NULL, SymFunctionTableAccess64, SymGetModuleBase64, NULL);

		unsigned char buffer[sizeof(IMAGEHLP_SYMBOL64) + 256];
		PIMAGEHLP_SYMBOL64 symbol = (PIMAGEHLP_SYMBOL64)&buffer;
		
		ZeroMemory(symbol, sizeof(IMAGEHLP_SYMBOL64) + 256);
		symbol->SizeOfStruct = sizeof(IMAGEHLP_SYMBOL64);
		symbol->MaxNameLength = 256;

		if (SymGetSymFromAddr64(process, stackframe.AddrPC.Offset, NULL, symbol)) {
      printf("[STACK] 0x%08I64X (%s+0x%I64X)\n",
        stackframe.AddrPC.Offset,
        symbol->Name,
        stackframe.AddrPC.Offset - symbol->Address);
    } else {
      printf("[STACK] ???\n");
    }
	}
	
	SymCleanup(process);
  */

}

#endif


