/* vim:set et sts=4: */

#include "ibus_afrim/engine.h"
#include "ibus_afrim/service.h"
#include <ibus.h>
#include <stdio.h>

static IBusBus *bus = NULL;
static IBusFactory *factory = NULL;

/* command line options */
static gboolean ibus = FALSE;
static gboolean verbose = FALSE;

static const GOptionEntry entries[] = {
    { "ibus", 'i', 0, G_OPTION_ARG_NONE, &ibus,
      "component is executed by ibus", NULL },
    { "verbose", 'v', 0, G_OPTION_ARG_NONE, &verbose, "verbose", NULL },
    { NULL },
};

static void
ibus_disconnected_cb (IBusBus *bus, gpointer user_data)
{
    ibus_quit ();
}

static void
init (void)
{
    ibus_init ();

    bus = ibus_bus_new ();
    g_object_ref_sink (bus);
    g_signal_connect (bus, "disconnected", G_CALLBACK (ibus_disconnected_cb),
                      NULL);

    factory = ibus_factory_new (ibus_bus_get_connection (bus));
    g_object_ref_sink (factory);
    ibus_factory_add_engine (factory, "afrim", IBUS_TYPE_Afrim_ENGINE);

    if (ibus)
        {
            ibus_bus_request_name (bus, "fodydev.IBus.afrim",
                                   0); // hangs if name doesn't match afrim.xml
        }
    else
        {
            IBusComponent *component;

            component = ibus_component_new (
                "fodydev.IBus.afrim", "Afrim Input Method", "0.1.0", "GPL",
                "Brady Fomegne <brady.fomegne@outlook.com>",
                "https://github.com/fodydev/ibus-afrim", "", "ibus-afrim");
            ibus_component_add_engine (
                component,
                ibus_engine_desc_new (
                    "afrim", "Afrim Input Method", "Afrim Input Method", "en",
                    "GPL", "Brady Fomegne <brady.fomegne@outlook.com>",
                    PKGDATADIR "/icons/ibus-enchant.svg", "us"));
            ibus_bus_register_component (bus, component);
        }
}

int
main (int argc, char **argv)
{

    GError *error = NULL;
    GOptionContext *context;

    /* Parse the command line */
    context = g_option_context_new ("- ibus afrim engine");
    g_option_context_add_main_entries (context, entries, "ibus-afrim");

    if (!g_option_context_parse (context, &argc, &argv, &error))
        {
            g_print ("Option parsing failed: %s\n", error->message);
            g_error_free (error);
            return (-1);
        }

    configure_logging ();

    /* Go */
    init ();
    ibus_main ();
}
