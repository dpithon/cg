#include <stdio.h>

#include "hit.h"
#include "log.h"
#include "material.h"
#include "pinhole.h"
#include "pixmap.h"
#include "pool.h"
#include "rgb.h"
#include "sampler.h"
#include "scene.h"
#include "shape.h"
#include "stack.h"

#define W	320
#define H	320

#define CONES	12
#define RADIUS 15
void build_scene(void)
{
	struct shape *s;
	struct coord loc1 = POINT_O, loc2 = POINT_O, loc3 = POINT_O;
	double angle = (2. * M_PI) / (double) CONES;

	struct altern_colors *cone_inside, *cone_outside;
	struct altern_colors *parab_inside, *parab_outside;
	struct altern_colors *plane_under, *plane_over;
	struct altern_colors *sphere_outside;

	cone_inside    = altern_colors(.9, rgb(RGBWhite), rgb(RGBOrange));
	cone_outside   = altern_colors(.4, rgb(RGBLightGray), rgb(RGBDarkBlue));
	parab_inside   = altern_colors(.1, rgb(RGBCyan), rgb(RGBGreen));
	parab_outside  = altern_colors(.3, rgb(RGBBlack), rgb(RGBBlue));
	plane_under    = altern_colors(5., rgb(RGBMediumOrchid1),
					   rgb(RGBGrey100));
	plane_over     = altern_colors(10., rgb(RGBCornflowerBlue),
					    rgb(RGBSnow));
	sphere_outside = altern_colors(22.5, rgb(RGBOrangeRed),
				           rgb(RGBGray100));

	for (int n = 0; n < CONES; n++) {
		double nangle = angle * (double) n;
		set_point(&loc1,
			cos(nangle) * RADIUS,
			0.,
			sin(nangle) * RADIUS
		);
		set_point(&loc2,
			cos(nangle) * (RADIUS + 4.),
			4.,
			sin(nangle) * (RADIUS + 4.)
		);
		set_point(&loc3,
			cos(nangle) * (RADIUS - 4.),
			1.,
			sin(nangle) * (RADIUS - 4.)
		);

		s = cone(&loc2, &loc1, 3.5);
		pat_circles(s, OUTSIDE, cone_outside);
		pat_circles(s, INSIDE, cone_inside);
		add_shape(s);

		s = paraboloid(&loc1, &loc3, 2.);
		pat_circles(s, INSIDE, parab_inside);
		pat_circles(s, OUTSIDE, parab_outside);
		add_shape(s);
	}

	s = plane(&point_o, &vector_j);
	pat_checker(s, OVER, plane_over);
	pat_checker(s, UNDER, plane_under);
	add_shape(s);

	rotate_x(90);
	translate(0, 0, -4);
	s = sphere(&point_o, 6);
	pat_sphchecker(s, OUTSIDE, sphere_outside);
	add_shape(s);
}


int main()
{
	double x, y, z, angle, radius;

	angle = 45.;
	radius = 30.;

	init_pixmap(W, H);
	build_scene();
	set_target(0, 0, 0);
	set_fov(55.);

	x = cos(angle) * radius;
	z = sin(angle) * radius;
	y = sin(angle) * 45.;
	set_location(x, y, z);
	if (!setup_pinhole())
		fatal("failed to setup pinhole camera");

	prepare_shape_matrices(pinhole_coord_system());

	for (int x = 0; x < W; x++)
		for (int y = 0; y < H; y++)
			sampler(x, y);

	write_pixmap("test.pnm");
	release_pixmap();

	return 0;
}
