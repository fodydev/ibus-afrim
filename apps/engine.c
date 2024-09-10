#include "ibus_afrim/engine.h"
#include "ibus_afrim/service.h"
#include <stdio.h>

/* functions prototype */
static void ibus_afrim_engine_class_init (IBusAfrimEngineClass *klass);
static void ibus_afrim_engine_init (IBusAfrimEngine *engine);
static void ibus_afrim_engine_destroy (IBusAfrimEngine *engine);
static void ibus_afrim_engine_enable (IBusEngine *engine);

G_DEFINE_TYPE (IBusAfrimEngine, ibus_afrim_engine, IBUS_TYPE_ENGINE)

static IBusEngineClass *parent_class = NULL;

static gboolean
can_get_surrounding_text (IBusAfrimEngine *afrim)
{
    return afrim->parent.client_capabilities & IBUS_CAP_SURROUNDING_TEXT;
}

static void
ibus_afrim_engine_class_init (IBusAfrimEngineClass *klass)
{
    IBusObjectClass *ibus_object_class = IBUS_OBJECT_CLASS (klass);
    IBusEngineClass *engine_class = IBUS_ENGINE_CLASS (klass);

    ibus_object_class->destroy
        = (IBusObjectDestroyFunc)ibus_afrim_engine_destroy;
    parent_class = (IBusEngineClass *)g_type_class_peek_parent (klass);

    engine_class->process_key_event = ibus_afrim_engine_process_key_event;
    engine_class->page_down = ibus_afrim_engine_page_down_button;
    engine_class->page_up = ibus_afrim_engine_page_up_button;
    engine_class->candidate_clicked = ibus_afrim_engine_candidate_clicked;
    engine_class->focus_out = ibus_afrim_engine_focus_out;
    engine_class->enable = ibus_afrim_engine_enable;
}

static void
ibus_afrim_engine_init (IBusAfrimEngine *afrim)
{
    afrim->engine_core = new_afrim_engine_core (afrim, parent_class);
    afrim->table = ibus_lookup_table_new (9, 0, TRUE, TRUE);
    g_object_ref_sink (afrim->table);
}

static void
ibus_afrim_engine_destroy (IBusAfrimEngine *afrim)
{
    if (afrim->table)
        {
            g_object_unref (afrim->table);
            afrim->table = NULL;
        }

    if (afrim->engine_core)
        {
            free_afrim_engine_core (afrim->engine_core);
            afrim->engine_core = NULL;
        }

    ((IBusObjectClass *)ibus_afrim_engine_parent_class)
        ->destroy ((IBusObject *)afrim);
}

static void
ibus_afrim_engine_enable (IBusEngine *engine)
{
    // dummy call to tell the input context that the engine will utilize
    // surrounding-text
    ibus_engine_get_surrounding_text (engine, NULL, NULL, NULL);
}
