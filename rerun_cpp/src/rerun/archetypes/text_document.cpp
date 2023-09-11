// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/text_document.fbs".

#include "text_document.hpp"

#include "../indicator_component.hpp"

namespace rerun {
    namespace archetypes {
        const char TextDocument::INDICATOR_COMPONENT_NAME[] =
            "rerun.components.TextDocumentIndicator";

        std::vector<AnonymousComponentBatch> TextDocument::as_component_batches() const {
            std::vector<AnonymousComponentBatch> comp_batches;
            comp_batches.reserve(1);

            comp_batches.emplace_back(body);
            comp_batches.emplace_back(
                ComponentBatch<
                    components::IndicatorComponent<TextDocument::INDICATOR_COMPONENT_NAME>>(
                    nullptr,
                    num_instances()
                )
            );

            return comp_batches;
        }
    } // namespace archetypes
} // namespace rerun