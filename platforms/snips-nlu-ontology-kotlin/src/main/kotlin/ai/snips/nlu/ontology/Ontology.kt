package ai.snips.nlu.ontology

import ai.snips.nlu.ontology.SlotValue.Type.*
import org.parceler.Parcel
import org.parceler.Parcel.Serialization.BEAN
import org.parceler.ParcelConstructor
import org.parceler.ParcelProperty

@Parcel(BEAN)
data class Range @ParcelConstructor constructor(@ParcelProperty("start") val start: Int,
                                                @ParcelProperty("end") val end: Int)

@Parcel(BEAN)
data class Slot @ParcelConstructor constructor(@ParcelProperty("rawValue") val rawValue: String,
                                               @ParcelProperty("value") val value: SlotValue,
                                               @ParcelProperty("range") val range: Range?,
                                               @ParcelProperty("entity") val entity: String,
                                               @ParcelProperty("slotName") val slotName: String)

enum class Precision { APPROXIMATE, EXACT }

enum class Grain { YEAR, QUARTER, MONTH, WEEK, DAY, HOUR, MINUTE, SECOND }

// TODO : add converters to JSR310 / ThreeTen types
sealed class SlotValue(val type: Type) {

    @Parcel
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
        MUSICALBUM,
        MUSICARTIST,
        MUSICTRACK
    }

    @Parcel(BEAN)
    data class CustomValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(CUSTOM)

    @Parcel(BEAN)
    data class NumberValue @ParcelConstructor constructor(@ParcelProperty("value") val value: Double) : SlotValue(NUMBER)

    @Parcel(BEAN)
    data class PercentageValue @ParcelConstructor constructor(@ParcelProperty("value") val value: Double) : SlotValue(PERCENTAGE)

    @Parcel(BEAN)
    data class OrdinalValue @ParcelConstructor constructor(@ParcelProperty("value") val value: Long) : SlotValue(ORDINAL)

    @Parcel(BEAN)
    data class InstantTimeValue @ParcelConstructor constructor(
            @ParcelProperty("value") val value: String,
            @ParcelProperty("grain") val grain: Grain,
            @ParcelProperty("precision") val precision: Precision) : SlotValue(INSTANT_TIME)

    @Parcel(BEAN)
    data class TimeIntervalValue @ParcelConstructor constructor(
            @ParcelProperty("from") val from: String?,
            @ParcelProperty("to") val to: String?) : SlotValue(TIME_INTERVAL)

    @Parcel(BEAN)
    data class AmountOfMoneyValue @ParcelConstructor constructor(
            @ParcelProperty("value") val value: Float,
            @ParcelProperty("precision") val precision: Precision,
            @ParcelProperty("unit") val unit: String?) : SlotValue(AMOUNT_OF_MONEY)

    @Parcel(BEAN)
    data class TemperatureValue @ParcelConstructor constructor(
            @ParcelProperty("value") val value: Float,
            @ParcelProperty("unit") val unit: String?) : SlotValue(TEMPERATURE)

    @Parcel(BEAN)
    data class DurationValue @ParcelConstructor constructor(
            @ParcelProperty("years") val years: Long,
            @ParcelProperty("quarters") val quarters: Long,
            @ParcelProperty("months") val months: Long,
            @ParcelProperty("weeks") val weeks: Long,
            @ParcelProperty("days") val days: Long,
            @ParcelProperty("hours") val hours: Long,
            @ParcelProperty("minutes") val minutes: Long,
            @ParcelProperty("seconds") val seconds: Long,
            @ParcelProperty("precision") val precision: Precision) : SlotValue(DURATION)

    @Parcel(BEAN)
    data class MusicAlbumValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(MUSICALBUM)

    @Parcel(BEAN)
    data class MusicArtistValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(MUSICARTIST)

    @Parcel(BEAN)
    data class MusicTrackValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(MUSICTRACK)
}


@Parcel(BEAN)
data class IntentClassifierResult @ParcelConstructor constructor(
        @ParcelProperty("intentName") val intentName: String,
        @ParcelProperty("probability") val probability: Float)

@Parcel(BEAN)
data class IntentParserResult @ParcelConstructor constructor(
        @ParcelProperty("input") val input: String,
        @ParcelProperty("intent") val intent: IntentClassifierResult?,
        @ParcelProperty("slots") val slots: List<Slot>)
