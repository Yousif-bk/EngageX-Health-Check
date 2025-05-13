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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://cxone.niceincontact.com/auth/authorize?response_type=code&scope=openid&client_id=0b697ebb-4ea2-4052-b12b-d3cf12a53eca&redirect_uri=https%3A%2F%2Fcxone.niceincontact.com%2Fua%2Fv1%2Fcallback&nonce=jl0DuqDJXF1rfsxk53CIpw%3D%3D')

WebUI.setText(findTestObject('Object Repository/Nice CXone Agent OR/NICE CXone Agent Login OR/input_Sign In_username'), '800engagex_aabaker@eand.com')

WebUI.click(findTestObject('Object Repository/Nice CXone Agent OR/NICE CXone Agent Login OR/button_NEXT'))

WebUI.setEncryptedText(findTestObject('Object Repository/Nice CXone Agent OR/NICE CXone Agent Login OR/input_Sign In_password'), 'rBxUql4cWJqz1D4sF7V5lQ==')

WebUI.click(findTestObject('Object Repository/Nice CXone Agent OR/NICE CXone Agent Login OR/button_Sign In'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Nice CXone Agent OR/Page_Employees/div_Employees'), 0)