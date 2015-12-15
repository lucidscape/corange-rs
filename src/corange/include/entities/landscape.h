/**
*** :: Light ::
***
***   -- WIP --
***
***   Object for rendering an instance of a terrain
***   asset using particular textures and attributes.
***
**/

#ifndef landscape_h
#define landscape_h

#include "../cengine.h"
#include "../casset.h"

#include "../assets/image.h"
#include "../assets/terrain.h"

typedef struct landscape_blobtree {

  sphere bound;

  bool is_leaf;
  int chunk_index;
  struct landscape_blobtree* child0;
  struct landscape_blobtree* child1;
  struct landscape_blobtree* child2;
  struct landscape_blobtree* child3;

} landscape_blobtree;

typedef struct {

  asset_hndl heightmap;
  asset_hndl attribmap;

  image* attribimage;

  float scale;
  float size_x;
  float size_y;
  landscape_blobtree* blobtree;

  asset_hndl ground0;
  asset_hndl ground1;
  asset_hndl ground2;
  asset_hndl ground3;

  asset_hndl ground0_nm;
  asset_hndl ground1_nm;
  asset_hndl ground2_nm;
  asset_hndl ground3_nm;

} landscape;

landscape* landscape_new();
void landscape_delete(landscape* l);

landscape_blobtree* landscape_blobtree_new(landscape* ls);
void landscape_blobtree_delete(landscape_blobtree* lbt);
void landscape_blobtree_generate(landscape* l);

mat4  landscape_world(landscape* l);
mat3  landscape_world_normal(landscape* l);
float landscape_height(landscape* l, vec2 pos);
vec3  landscape_normal(landscape* l, vec2 pos);
mat3  landscape_axis(landscape* l, vec2 pos);

void landscape_paint_height(landscape* l, vec2 pos, float radius, float value, float opacity);
void landscape_paint_color(landscape* l, vec2 pos, float radius, int type, float opacity);

void landscape_chunks(landscape* l, vec2 pos, struct terrain_chunk** chunks_out);

#endif
