package ai.snips.nlu.ontology

import ai.snips.nlu.ontology.SlotValue.Type.AMOUNT_OF_MONEY
import ai.snips.nlu.ontology.SlotValue.Type.CUSTOM
import ai.snips.nlu.ontology.SlotValue.Type.DURATION
import ai.snips.nlu.ontology.SlotValue.Type.INSTANT_TIME
import ai.snips.nlu.ontology.SlotValue.Type.NUMBER
import ai.snips.nlu.ontology.SlotValue.Type.ORDINAL
import ai.snips.nlu.ontology.SlotValue.Type.TEMPERATURE
import ai.snips.nlu.ontology.SlotValue.Type.TIME_INTERVAL
import ai.snips.nlu.ontology.SlotValue.Type.PERCENTAGE

data class Range(val start: Int, val end: Int)

data class Slot(val rawValue: String, val value: SlotValue, val range: Range?, val entity: String, val slotName: String)

enum class Precision {APPROXIMATE, EXACT }
enum class Grain { YEAR, QUARTER, MONTH, WEEK, DAY, HOUR, MINUTE, SECOND }

// TODO : add converters to JSR310 / ThreeTen types
sealed class SlotValue(val type: SlotValue.Type) {

    enum class Type {
        CUSTOM,
        NUMBER,
        ORDINAL,
        INSTANT_TIME,
        TIME_INTERVAL,
        AMOUNT_OF_MONEY,
        TEMPERATURE,
        DURATION,
        PERCENTAGE,
    }

    data class CustomValue(val value: String) : SlotValue(CUSTOM)
    data class NumberValue(val value: Double) : SlotValue(NUMBER)
    data class PercentageValue(val value: Double) : SlotValue(PERCENTAGE)
    data class OrdinalValue(val value: Long) : SlotValue(ORDINAL)
    data class InstantTimeValue(val value: String, val grain: Grain, val precision: Precision) : SlotValue(INSTANT_TIME)
    data class TimeIntervalValue(val from: String, val to: String) : SlotValue(TIME_INTERVAL)
    data class AmountOfMoneyValue(val value: Float, val precision: Precision, val unit: String) : SlotValue(AMOUNT_OF_MONEY)
    data class TemperatureValue(val value: Float, val unit: String) : SlotValue(TEMPERATURE)
    data class DurationValue(val years: Long,
                             val quarters: Long,
                             val months: Long,
                             val weeks: Long,
                             val days: Long,
                             val hours: Long,
                             val minutes: Long,
                             val seconds: Long,
                             val precision: Precision) : SlotValue(DURATION)
}


data class IntentClassifierResult(val intentName: String, val probability: Float)
data class IntentParserResult(val input: String, val intent: IntentClassifierResult?, val slots: List<Slot>)
