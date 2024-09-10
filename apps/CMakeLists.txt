add_executable(ibus-engine-afrim
        main.c
        engine.c
)
target_link_libraries(ibus-engine-afrim PUBLIC ${IBUS_LIBRARIES} service)

target_include_directories(ibus-engine-afrim PUBLIC ${IBUS_INCLUDE_DIRS})
target_include_directories(ibus-engine-afrim PUBLIC ../include)

configure_file(afrim.xml.in afrim.xml)

install(TARGETS ibus-engine-afrim DESTINATION ${LIBEXECDIR})
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/afrim.xml DESTINATION ${PKGDATADIR}/ibus/component)
