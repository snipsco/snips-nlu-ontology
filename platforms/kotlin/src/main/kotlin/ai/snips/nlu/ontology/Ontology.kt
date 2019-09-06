package ai.snips.nlu.ontology

import ai.snips.nlu.ontology.SlotValue.AmountOfMoneyValue
import ai.snips.nlu.ontology.SlotValue.CityValue
import ai.snips.nlu.ontology.SlotValue.CountryValue
import ai.snips.nlu.ontology.SlotValue.CustomValue
import ai.snips.nlu.ontology.SlotValue.DurationValue
import ai.snips.nlu.ontology.SlotValue.InstantTimeValue
import ai.snips.nlu.ontology.SlotValue.MusicAlbumValue
import ai.snips.nlu.ontology.SlotValue.MusicArtistValue
import ai.snips.nlu.ontology.SlotValue.MusicTrackValue
import ai.snips.nlu.ontology.SlotValue.NumberValue
import ai.snips.nlu.ontology.SlotValue.OrdinalValue
import ai.snips.nlu.ontology.SlotValue.PercentageValue
import ai.snips.nlu.ontology.SlotValue.RegionValue
import ai.snips.nlu.ontology.SlotValue.TemperatureValue
import ai.snips.nlu.ontology.SlotValue.TimeIntervalValue
import ai.snips.nlu.ontology.SlotValue.Type.AMOUNT_OF_MONEY
import ai.snips.nlu.ontology.SlotValue.Type.CITY
import ai.snips.nlu.ontology.SlotValue.Type.COUNTRY
import ai.snips.nlu.ontology.SlotValue.Type.CUSTOM
import ai.snips.nlu.ontology.SlotValue.Type.DURATION
import ai.snips.nlu.ontology.SlotValue.Type.INSTANT_TIME
import ai.snips.nlu.ontology.SlotValue.Type.MUSICALBUM
import ai.snips.nlu.ontology.SlotValue.Type.MUSICARTIST
import ai.snips.nlu.ontology.SlotValue.Type.MUSICTRACK
import ai.snips.nlu.ontology.SlotValue.Type.NUMBER
import ai.snips.nlu.ontology.SlotValue.Type.ORDINAL
import ai.snips.nlu.ontology.SlotValue.Type.PERCENTAGE
import ai.snips.nlu.ontology.SlotValue.Type.REGION
import ai.snips.nlu.ontology.SlotValue.Type.TEMPERATURE
import ai.snips.nlu.ontology.SlotValue.Type.TIME_INTERVAL
import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonSubTypes.Type
import com.fasterxml.jackson.annotation.JsonTypeInfo
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
                                               //use a MutableList here to make parceler happy
                                               @ParcelProperty("alternatives") val alternatives: MutableList<SlotValue>,
                                               @ParcelProperty("range") val range: Range,
                                               @ParcelProperty("entity") val entity: String,
                                               @ParcelProperty("slotName") val slotName: String,
                                               @ParcelProperty("confidenceScore") val confidenceScore: Float?)

enum class Precision { APPROXIMATE, EXACT }

enum class Grain { YEAR, QUARTER, MONTH, WEEK, DAY, HOUR, MINUTE, SECOND }

// TODO : add converters to JSR310 / ThreeTen types
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.EXISTING_PROPERTY, property = "kind")
@JsonSubTypes(
        Type(value = CustomValue::class, name = "Custom"),
        Type(value = NumberValue::class, name = "Number"),
        Type(value = OrdinalValue::class, name = "Ordinal"),
        Type(value = InstantTimeValue::class, name = "InstantTime"),
        Type(value = TimeIntervalValue::class, name = "TimeInterval"),
        Type(value = AmountOfMoneyValue::class, name = "AmountOfMoney"),
        Type(value = TemperatureValue::class, name = "Temperature"),
        Type(value = DurationValue::class, name = "Duration"),
        Type(value = PercentageValue::class, name = "Percentage"),
        Type(value = MusicAlbumValue::class, name = "MusicAlbum"),
        Type(value = MusicArtistValue::class, name = "MusicArtist"),
        Type(value = MusicTrackValue::class, name = "MusicTrack"),
        Type(value = CityValue::class, name = "City"),
        Type(value = CountryValue::class, name = "Country"),
        Type(value = RegionValue::class, name = "Region")
)
sealed class SlotValue(val kind: Type) {

    @JsonIgnore
    @Deprecated("use kind instead")
    val type = kind

    @Parcel
    enum class Type {
        @JsonProperty("Custom") CUSTOM,
        @JsonProperty("Number") NUMBER,
        @JsonProperty("Ordinal") ORDINAL,
        @JsonProperty("InstantTime") INSTANT_TIME,
        @JsonProperty("TimeInterval") TIME_INTERVAL,
        @JsonProperty("AmountOfMoney") AMOUNT_OF_MONEY,
        @JsonProperty("Temperature") TEMPERATURE,
        @JsonProperty("Duration") DURATION,
        @JsonProperty("Percentage") PERCENTAGE,
        @JsonProperty("MusicAlbum") MUSICALBUM,
        @JsonProperty("MusicArtist") MUSICARTIST,
        @JsonProperty("MusicTrack") MUSICTRACK,
        @JsonProperty("City") CITY,
        @JsonProperty("Country") COUNTRY,
        @JsonProperty("Region") REGION
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

    @Parcel(BEAN)
    data class CityValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(CITY)

    @Parcel(BEAN)
    data class CountryValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(COUNTRY)

    @Parcel(BEAN)
    data class RegionValue @ParcelConstructor constructor(@ParcelProperty("value") val value: String) : SlotValue(REGION)
}

@Parcel(BEAN)
data class IntentClassifierResult @ParcelConstructor constructor(
        @ParcelProperty("intentName") val intentName: String?,
        @ParcelProperty("confidenceScore") val confidenceScore: Float)

@Parcel(BEAN)
data class IntentParserAlternative @ParcelConstructor constructor(
        @ParcelProperty("intent") val intent: IntentClassifierResult,
        @ParcelProperty("slots") val slots: List<Slot>)

@Parcel(BEAN)
data class IntentParserResult @ParcelConstructor constructor(
        @ParcelProperty("input") val input: String,
        @ParcelProperty("intent") val intent: IntentClassifierResult,
        @ParcelProperty("slots") val slots: List<Slot>,
        @ParcelProperty("alternatives") val alternatives: List<IntentParserAlternative>)
