#ifndef SHAPE_H
#define SHAPE_H

#include <stddef.h>

#include "slist.h"
#include "types.h"
#include "vmath.h"

#define SHAPE(s)  ((struct shape*)(s))

struct shape {
	struct slink slink; // slink MUST be the first field !

	int shape_type;

	struct cs cs;
	struct matrix cam_to_shp;
	struct matrix shp_to_cam;

	struct material *material[2];
};

#define foreach_shape(s, shape_list) \
	struct slist_iterator UNIQUE_NAME(iter);\
	init_slist_iterator(&UNIQUE_NAME(iter), SLIST(shape_list));\
	foreach (SHAPE, s, &UNIQUE_NAME(iter))

extern struct shape *plane(const struct coord*, const struct coord*);
extern struct shape *sphere(const struct coord*, double);
extern struct shape *cylinder(const struct coord*, const struct coord*, double);
extern struct shape *cone(const struct coord*, const struct coord*, double);
extern struct shape *paraboloid(const struct coord*, const struct coord*,
				double);

extern bool intersect(struct hit*, const struct ray*, const struct shape*);
extern void normal(struct coord*, const struct hit*);

#endif /* SHAPE_H */
