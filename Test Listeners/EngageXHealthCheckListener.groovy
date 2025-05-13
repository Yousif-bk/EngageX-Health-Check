import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext
import com.kms.katalon.core.annotation.*
import groovy.json.JsonOutput
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.RequestObject

class EngageXHealthCheckListener {
	private static List<String> testResults = []
	
		@BeforeTestCase
		def sampleBeforeTestCase(TestCaseContext testCaseContext) {
			println "Starting test case: " + testCaseContext.getTestCaseId()
		}
	
		@AfterTestCase
		def sampleAfterTestCase(TestCaseContext testCaseContext) {
			String testCaseId = testCaseContext.getTestCaseId()
		String testStatus = testCaseContext.getTestCaseStatus()
		
		println "Adding Test Case Result: ${testCaseId}: ${testStatus}"
		
		// Add only once
		testResults.add("- ${testCaseId}: *${testStatus}*")
		
		WS.comment("Test Results: " + testResults)
		}
	
		@AfterTestSuite
		def sampleAfterTestSuite(TestSuiteContext testSuiteContext) {
			println "Test Suite Finished: " + testSuiteContext.getTestSuiteId()
			def testSuiteId = testSuiteContext.getTestSuiteId()
			def testResultsText = testResults.join("\n")
	
			def summary = "EngageX Healh Check Report"
			def description = "Test Execution Summary:\n\n${testResultsText}"
	
			WS.comment("Test summary: " + summary)
			WS.comment("Test description: " + description)
			
			boolean hasFailure = testResultsText.toLowerCase().contains("failed")
			def customFieldValue = hasFailure ? "Failed" : "Pass"
			WS.comment("Test customFieldValue: " + customFieldValue)
			
			RequestObject jiraRequest = ObjectRepository.findTestObject('JIRA/Create Issue')
		
		 String requestBody = JsonOutput.toJson([
		 serviceDeskId       : "9",
			requestTypeId       : "456",
			requestFieldValues  : [
				summary         : summary,
				description     : description,
				customfield_10132: [
					value       : "Technical Issue",
					child       : [value: "Alarm"]
				],
				priority        : [name: "Low"],
				customfield_10334: [value: "Bot"],
				customfield_10171: customFieldValue,
			]
		])
			
			jiraRequest.setBodyContent(new com.kms.katalon.core.testobject.impl.HttpTextBodyContent(requestBody, "UTF-8", "application/json"))
			
			def response = WS.sendRequest(jiraRequest)
		
			println "Jira Response: " + response.getResponseBodyContent()
			
			WS.comment("Jira Response: " + response.getResponseBodyContent())
		}
	
}