/* -*- Mode: C; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* vim: set sw=4 sts=4 ts=4 expandtab: */

#ifndef TEST_UTILS_H
#define TEST_UTILS_H

#include <cairo.h>
#include <gio/gio.h>
#include <gdk-pixbuf/gdk-pixbuf.h>

G_BEGIN_DECLS

typedef struct {
    unsigned int pixels_changed;
    unsigned int max_diff;
} TestUtilsBufferDiffResult;

void test_utils_compare_surfaces (cairo_surface_t           *surface_a,
                                  cairo_surface_t           *surface_b,
                                  cairo_surface_t           *surface_diff,
                                  TestUtilsBufferDiffResult *result);

cairo_surface_t *test_utils_cairo_surface_from_pixbuf (const GdkPixbuf *pixbuf);

typedef gboolean (* AddTestFunc) (GFile *file);

const gchar *test_utils_get_test_data_path      (void);

void         test_utils_add_test_for_all_files  (const gchar    *prefix,
                                                 GFile          *base,
                                                 GFile          *file,
                                                 GTestDataFunc   test_func,
                                                 AddTestFunc     add_test_func);
G_END_DECLS

#endif /* TEST_UTILS_H */
