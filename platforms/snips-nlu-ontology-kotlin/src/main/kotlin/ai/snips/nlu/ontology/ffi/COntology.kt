package ai.snips.nlu.ontology.ffi

import ai.snips.nlu.ontology.Grain
import ai.snips.nlu.ontology.IntentClassifierResult
import ai.snips.nlu.ontology.IntentParserResult
import ai.snips.nlu.ontology.Precision
import ai.snips.nlu.ontology.Range
import ai.snips.nlu.ontology.Slot
import ai.snips.nlu.ontology.SlotValue
import ai.snips.nlu.ontology.SlotValue.AmountOfMoneyValue
import ai.snips.nlu.ontology.SlotValue.CustomValue
import ai.snips.nlu.ontology.SlotValue.DurationValue
import ai.snips.nlu.ontology.SlotValue.InstantTimeValue
import ai.snips.nlu.ontology.SlotValue.MusicAlbumValue
import ai.snips.nlu.ontology.SlotValue.MusicArtistValue
import ai.snips.nlu.ontology.SlotValue.MusicTrackValue
import ai.snips.nlu.ontology.SlotValue.NumberValue
import ai.snips.nlu.ontology.SlotValue.PercentageValue
import ai.snips.nlu.ontology.SlotValue.OrdinalValue
import ai.snips.nlu.ontology.SlotValue.TemperatureValue
import ai.snips.nlu.ontology.SlotValue.TimeIntervalValue
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.toJnaPointer


const val RUST_ENCODING = "utf-8"

fun Pointer?.readString(): String = this!!.getString(0, RUST_ENCODING)
fun String.toPointer(): Pointer = this.toJnaPointer(RUST_ENCODING)
fun Int?.readGrain(): Grain = CGrain.toGrain(this!!)
fun Int?.readPrecision(): Precision = CPrecision.toPrecision(this!!)
fun Int?.readRangeTo(end: Int?): Range = Range(this!!, end!!)
fun CSlotValue?.readSlotValue(): SlotValue = this!!.toSlotValue()

class CIntentParserResult(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var input: Pointer? = null
    @JvmField var intent: CIntentClassifierResult? = null
    @JvmField var slots: CSlots? = null

    override fun getFieldOrder() = listOf("input",
                                          "intent",
                                          "slots")

    fun toIntentParserResult() = IntentParserResult(input = input.readString(),
                                                    intent = intent!!.toIntentClassifierResult(),
                                                    slots = slots!!.toSlotList())

}

class CIntentClassifierResult : Structure(), Structure.ByReference {
    @JvmField var intent_name: Pointer? = null
    @JvmField var probability: Float? = null

    override fun getFieldOrder() = listOf("intent_name", "probability")

    fun toIntentClassifierResult() = IntentClassifierResult(intentName = intent_name?.readString(),
                                                            probability = probability!!)
}

class CSlots : Structure(), Structure.ByReference {

    @JvmField var slots: Pointer? = null
    @JvmField var size: Int = -1

    override fun getFieldOrder() = listOf("slots", "size")

    fun toSlotList(): List<Slot> = CSlot(slots!!).toArray(size).map { (it as CSlot).toSlot() }
}

object CGrain {
    const val YEAR = 0
    const val QUARTER = 1
    const val MONTH = 2
    const val WEEK = 3
    const val DAY = 4
    const val HOUR = 5
    const val MINUTE = 6
    const val SECOND = 7

    fun toGrain(input: Int) = when (input) {
        YEAR -> Grain.YEAR
        QUARTER -> Grain.QUARTER
        MONTH -> Grain.MONTH
        WEEK -> Grain.WEEK
        DAY -> Grain.DAY
        HOUR -> Grain.HOUR
        MINUTE -> Grain.MINUTE
        SECOND -> Grain.SECOND
        else -> throw IllegalArgumentException("unknown grain $input")
    }
}

object CPrecision {
    const val APPROXIMATE = 0
    const val EXACT = 1

    fun toPrecision(input: Int) = when (input) {
        APPROXIMATE -> Precision.APPROXIMATE
        EXACT -> Precision.EXACT
        else -> throw IllegalArgumentException("unknown precision $input")
    }
}

class CSlotValue : Structure(), Structure.ByValue {
    companion object {
        const val CUSTOM = 1
        const val NUMBER = 2
        const val ORDINAL = 3
        const val INSTANTTIME = 4
        const val TIMEINTERVAL = 5
        const val AMOUNTOFMONEY = 6
        const val TEMPERATURE = 7
        const val DURATION = 8
        const val PERCENTAGE = 9
        const val MUSICALBUM = 10
        const val MUSICARTIST = 11
        const val MUSICTRACK = 12
    }

    @JvmField var value_type: Int? = null
    @JvmField var value: Pointer? = null

    override fun getFieldOrder() = listOf("value", "value_type")

    fun toSlotValue(): SlotValue = when (value_type!!) {
        CUSTOM -> CustomValue(value.readString())
        NUMBER -> NumberValue(value!!.getDouble(0))
        ORDINAL -> OrdinalValue(value!!.getLong(0))
        INSTANTTIME -> CInstantTimeValue(value!!).toInstantTimeValue()
        TIMEINTERVAL -> CTimeIntervalValue(value!!).toTimeIntervalValue()
        AMOUNTOFMONEY -> CAmountOfMoneyValue(value!!).toAmountOfMoneyValue()
        TEMPERATURE -> CTemperatureValue(value!!).toTemperatureValue()
        DURATION -> CDurationValue(value!!).toDurationValue()
        PERCENTAGE -> PercentageValue(value!!.getDouble(0))
        MUSICALBUM -> MusicAlbumValue(value.readString())
        MUSICARTIST -> MusicArtistValue(value.readString())
        MUSICTRACK -> MusicTrackValue(value.readString())
        else -> throw IllegalArgumentException("unknown value type $value_type")
    }

}

class CInstantTimeValue(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var value: Pointer? = null
    @JvmField var grain: Int? = null
    @JvmField var precision: Int? = null
    override fun getFieldOrder() = listOf("value", "grain", "precision")
    fun toInstantTimeValue(): InstantTimeValue {
        return InstantTimeValue(value = value.readString(),
                                grain = grain.readGrain(),
                                precision = precision.readPrecision())

    }
}

class CTimeIntervalValue(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var from: Pointer? = null
    @JvmField var to: Pointer? = null
    override fun getFieldOrder() = listOf("from", "to")
    fun toTimeIntervalValue() = TimeIntervalValue(from = from?.readString(), to = to?.readString())
}

class CAmountOfMoneyValue(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var value: Float? = null
    @JvmField var precision: Int? = null
    @JvmField var unit: Pointer? = null

    override fun getFieldOrder() = listOf("unit", "value", "precision")

    fun toAmountOfMoneyValue() = AmountOfMoneyValue(value = value!!,
                                                    precision = precision.readPrecision(),
                                                    unit = unit?.readString())
}

class CTemperatureValue(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var value: Float? = null
    @JvmField var unit: Pointer? = null

    override fun getFieldOrder() = listOf("unit", "value")

    fun toTemperatureValue() = TemperatureValue(value = value!!,
                                                unit = unit?.readString())

}

class CDurationValue(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var years: Long? = null
    @JvmField var quarters: Long? = null
    @JvmField var months: Long? = null
    @JvmField var weeks: Long? = null
    @JvmField var days: Long? = null
    @JvmField var hours: Long? = null
    @JvmField var minutes: Long? = null
    @JvmField var seconds: Long? = null
    @JvmField var precision: Int? = null

    override fun getFieldOrder() = listOf("years",
                                          "quarters",
                                          "months",
                                          "weeks",
                                          "days",
                                          "hours",
                                          "minutes",
                                          "seconds",
                                          "precision")

    fun toDurationValue() = DurationValue(years = years!!,
                                          quarters = quarters!!,
                                          months = months!!,
                                          weeks = weeks!!,
                                          days = days!!,
                                          hours = hours!!,
                                          minutes = minutes!!,
                                          seconds = seconds!!,
                                          precision = precision.readPrecision())
}


class CSlot(p: Pointer) : Structure(p), Structure.ByReference {
    init {
        read()
    }

    @JvmField var raw_value: Pointer? = null
    @JvmField var value: CSlotValue? = null
    @JvmField var range_start: Int? = null
    @JvmField var range_end: Int? = null
    @JvmField var entity: Pointer? = null
    @JvmField var slot_name: Pointer? = null

    override fun getFieldOrder() = listOf("value",
                                          "raw_value",
                                          "entity",
                                          "slot_name",
                                          "range_start",
                                          "range_end")

    fun toSlot() = Slot(rawValue = raw_value.readString(),
                        value = value.readSlotValue(),
                        range = range_start.readRangeTo(range_end),
                        entity = entity.readString(),
                        slotName = slot_name.readString())
}
