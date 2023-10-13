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
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

// Define expected variables
response = WS.sendRequest(findTestObject('User Management/Login', [('BASE_URL') : GlobalVariable.BASE_URL, ('EMAIL') : GlobalVariable.EMAIL
            , ('PASS') : GlobalVariable.PASS]))

WS.verifyResponseStatusCode(response, 200, FailureHandling.STOP_ON_FAILURE)

// Parse the JSON response
def jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseBodyContent())

// Extract values from the response
def emailFromResponse = jsonResponse.user.email
def nameFromResponse = jsonResponse.user.name
def roleFromResponse = jsonResponse.user.role
def accessToken = jsonResponse.access_token
def refreshToken = jsonResponse.refresh_token

// Verify a specific element in the response (Continue on Failure)
def proplist = [emailFromResponse, nameFromResponse, roleFromResponse]
def expectedResult = [GlobalVariable.EMAIL, GlobalVariable.NAME, GlobalVariable.ROLE]
for (int i = 0; i < proplist.size(); i++) {
	WS.verifyEqual(proplist[i], expectedResult[i], FailureHandling.CONTINUE_ON_FAILURE)
}

// Verify access token and refresh token must be exist
def checkToken = [accessToken, refreshToken]
for (int i = 0; i < checkToken.size(); i++) {
	WS.verifyNotEqual(checkToken[i], "", FailureHandling.STOP_ON_FAILURE)
}

// Store specific values in global variables
GlobalVariable.authToken = accessToken
println('extracted here : ' + accessToken)