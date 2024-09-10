#ifndef __ENGINE_H__
#define __ENGINE_H__

#include <ibus.h>

typedef struct _IBusAfrimEngine IBusAfrimEngine;
typedef struct _IBusAfrimEngineClass IBusAfrimEngineClass;
typedef struct EngineCore EngineCore;

static gboolean GBOOL_FALSE = FALSE;
static gboolean GBOOL_TRUE = TRUE;

struct _IBusAfrimEngine
{
    IBusEngine parent;
    IBusLookupTable *table;

    EngineCore *engine_core;
};

struct _IBusAfrimEngineClass
{
    IBusEngineClass parent;
};

#define IBUS_TYPE_Afrim_ENGINE (ibus_afrim_engine_get_type ())

GType ibus_afrim_engine_get_type (void);

#endif
