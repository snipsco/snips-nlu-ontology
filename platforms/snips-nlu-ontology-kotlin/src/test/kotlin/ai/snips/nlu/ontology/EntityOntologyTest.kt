package ai.snips.nlu.ontology

import com.google.common.truth.Truth.assertThat
import org.junit.Test
import org.junit.Assert.fail


class EntityOntologyTest {

    @Test
    fun completeEntityOntologyJsonWorks() {
        assertThat(EntityOntology.completeEntityOntologyJson()).isNotEmpty()
    }

    @Test
    fun entityOntologyJsonWorks() {
        assertThat(EntityOntology.entityOntologyJson("en")).isNotEmpty()
    }

    @Test
    fun entityOntologyJsonRaisesErrorOnUnknownLanguage() {
        try {
            EntityOntology.entityOntologyJson("klingon")
            fail("Should raise exception for unknown language")
        } catch (e: RuntimeException) {
            assertThat(e.message).contains("Unknown language")
        }
    }
}
