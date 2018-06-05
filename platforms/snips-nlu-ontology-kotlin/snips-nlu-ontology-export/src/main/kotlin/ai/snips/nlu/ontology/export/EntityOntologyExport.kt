package ai.snips.nlu.ontology.export

import ai.snips.nlu.ontology.export.EntityOntology.SnipsNluOntologyClientLibrary.Companion.INSTANCE as LIB
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.ptr.PointerByReference
import com.sun.jna.toJnaPointer

const val RUST_ENCODING = "utf-8"

fun String.toPointer(): Pointer = this.toJnaPointer(RUST_ENCODING)

class EntityOntology {
    companion object {
        private fun parseError(returnCode: Int) {
            if (returnCode != 0) {
                PointerByReference().apply {
                    LIB.snips_nlu_ontology_get_last_error(this)
                    throw RuntimeException(value.getString(0).apply {
                        LIB.snips_nlu_ontology_destroy_string(value)
                    })
                }
            }
        }

        @JvmStatic
        fun completeEntityOntologyJson(): String = PointerByReference().run {
            parseError(LIB.snips_nlu_ontology_complete_entity_ontology_json(this))
            value.getString(0).apply { LIB.snips_nlu_ontology_destroy_string(value) }
        }

        @JvmStatic
        fun languageEntityOntologyJson(language: String): String = PointerByReference().run {
            parseError(LIB.snips_nlu_ontology_language_entity_ontology_json(language.toPointer(), this))
            value.getString(0).apply { LIB.snips_nlu_ontology_destroy_string(value) }
        }
    }

    internal interface SnipsNluOntologyClientLibrary : Library {
        companion object {
            val INSTANCE: SnipsNluOntologyClientLibrary = Native.loadLibrary("snips_nlu_ontology_ffi", SnipsNluOntologyClientLibrary::class.java)
        }

        fun snips_nlu_ontology_complete_entity_ontology_json(result: PointerByReference): Int
        fun snips_nlu_ontology_language_entity_ontology_json(language: Pointer, result: PointerByReference): Int
        fun snips_nlu_ontology_get_last_error(error: PointerByReference): Int
        fun snips_nlu_ontology_destroy_string(string: Pointer): Int
    }
}
