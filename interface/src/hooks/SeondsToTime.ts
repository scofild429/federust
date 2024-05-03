const SecondsToTime = (seconds: number) => {
	var tmphour = Math.floor(seconds / 60 / 60)
	var tmpminute = Math.floor((seconds / 60) % 60)
	var tmpsecond = Math.floor(seconds % 60)

	var hour = ""
	var minute = ""
	var second = ""

	if (Math.floor(tmphour / 10) == 0) {
		hour = '0' + tmphour
	} else {
		hour = '' + tmphour
	}
	if (Math.floor(tmpminute / 10) == 0) {
		minute = '0' + tmpminute
	} else {
		minute = '' + tmpminute
	}

	if (Math.floor(tmpsecond / 10) == 0) {
		second = '0' + tmpsecond
	} else {
		second = '' + tmpsecond
	}

	var time = hour + ':' + minute + ':' + second + ''
	return time
}
export default SecondsToTime

