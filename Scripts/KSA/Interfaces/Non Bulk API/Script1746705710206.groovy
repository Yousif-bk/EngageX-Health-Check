import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper
import com.kms.katalon.core.util.KeywordUtil


def authResponse = WS.sendRequest(findTestObject('KSA/Interfaces/Auth API OR'))

if (authResponse.getStatusCode() != 200) {
	KeywordUtil.markFailedAndStop("Auth API failed! Status: " + authResponse.getStatusCode())
}

def jsonResponse = new JsonSlurper().parseText(authResponse.getResponseBodyContent())
def token = jsonResponse.access_token

println("Extracted Token: " + token)
WS.comment("Extracted Token: " + token)

def smsResponse = WS.sendRequest(findTestObject('KSA/Interfaces/Non Bulk API OR', [('authToken') : token]))

if (smsResponse.getStatusCode() == 200 || smsResponse.getStatusCode() == 202) {
	KeywordUtil.logInfo("Email API Call Successful! Status: " + smsResponse.getStatusCode())
} else {
	KeywordUtil.markFailed("Email API Call Failed! Status: " + smsResponse.getStatusCode())
}


println("Submit SMS Response: " + smsResponse.getResponseBodyContent())
WS.comment("Submit SMS Response: " + smsResponse.getResponseBodyContent())