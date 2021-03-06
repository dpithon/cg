#ifndef TYPES_H
#define TYPES_H

#include <stdbool.h>

struct coord;
struct cs;
struct hit;
struct material;
struct matrix;
struct quadratic;
struct ray;
struct rgb;
struct shape;
struct sphcoord;

typedef void (*render_f)(struct rgb*, const struct hit*);
typedef void (*sample_f)(int, int);
typedef void (*intrinsic_f)(struct rgb*, const struct hit*,
			    const struct material*);
typedef bool (*intersect_f)(struct hit*, const struct ray*,
			    const struct shape*);
typedef void (*normal_f)(struct coord*, const struct coord*);

#endif /* TYPES_H */
