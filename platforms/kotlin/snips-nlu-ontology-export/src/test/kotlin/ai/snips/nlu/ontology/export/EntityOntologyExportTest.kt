package ai.snips.nlu.ontology.export

import com.google.common.truth.Truth.assertThat
import org.junit.Test
import org.junit.Assert.fail


class EntityOntologyExportTest {

    @Test
    fun completeEntityOntologyJsonWorks() {
        assertThat(EntityOntology.completeEntityOntologyJson()).isNotEmpty()
    }

    @Test
    fun entityOntologyJsonWorks() {
        assertThat(EntityOntology.languageEntityOntologyJson("en")).isNotEmpty()
    }

    @Test
    fun entityOntologyJsonRaisesErrorOnUnknownLanguage() {
        try {
            EntityOntology.languageEntityOntologyJson("klingon")
            fail("Should raise exception for unknown language")
        } catch (e: RuntimeException) {
            assertThat(e.message).contains("Unknown language")
        }
    }
}
