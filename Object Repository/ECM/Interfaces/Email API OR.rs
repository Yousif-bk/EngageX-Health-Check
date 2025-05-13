<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Email API OR</name>
   <tag></tag>
   <elementGuidId>34277080-e953-46e3-aa9f-3d2371f4a26e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;  {\n \&quot;campaignName\&quot;: \&quot;Email campaign\&quot;,\n  \&quot;desc\&quot;: \&quot;Email campaign\&quot;,\n  \&quot;msgCategory\&quot;: \&quot;4.5\&quot;,\n  \&quot;dndCategory\&quot;: \&quot;\&quot;,\n  \&quot;senderAddr\&quot;: \&quot;\&quot;,\n  \&quot;priority\&quot;: \&quot;1\&quot;,\n  \&quot;schTime\&quot;: \&quot;\&quot;,\n  \&quot;expiryDt\&quot;: \&quot;\&quot;,\n    \&quot;clientTxnId\&quot;: \&quot;\&quot;,\n  \&quot;subject\&quot;: \&quot;EngageX Health Check: ECM Email API Test\&quot;,\n  \&quot;fallbackMsg\&quot;: \&quot;\&quot;,\n  \&quot;emailBody\&quot;: \&quot;ECM Email API Test\&quot;,\n  \&quot;senderDisplayName\&quot;: \&quot;TEST EMAIL\&quot;,\n  \&quot;smsFallbackType\&quot;: \&quot;3\&quot;,\n   \&quot;dr\&quot;: \&quot;1\&quot;,\n  \&quot;recipients\&quot;: [\n    \&quot;yusuf.babiker0@mail.com\&quot;\n  ],\n   \n  \&quot;emailFrom\&quot;: \&quot;engagex.operations@engagex.ae\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>56728314-cdf1-44d6-82ea-7a124c7006f3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${jwt_token}</value>
      <webElementGuid>f91580f2-4568-45da-bd03-274fd123abb7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://smartmessaging.etisalat.ae:5676/campaigns/submissions/email/b/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4cd09bc8-1f1b-46f1-bddd-d792d4917485</id>
      <masked>false</masked>
      <name>jwt_token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
