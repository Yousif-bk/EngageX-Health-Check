<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Bulk API OR</name>
   <tag></tag>
   <elementGuidId>6de03709-fc78-4111-8c7e-33363ad8a2e9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n \&quot;msgCategory\&quot;: \&quot;4.5\&quot;,\n \&quot;contentType\&quot;: \&quot;3.7\&quot;,\n \&quot;senderAddr\&quot;: \&quot;engageX\&quot;,\n \&quot;dndCategory\&quot;: \&quot;Campaign\&quot;,\n \&quot;priority\&quot;: 1,\n \&quot;clientTxnId\&quot;: \&quot;112346587963\&quot;,\n \&quot;schTime\&quot;: \&quot;\&quot;,\n \&quot;expiryDt\&quot;: \&quot;\&quot;,\n \&quot;desc\&quot;: \&quot;This is the description for campaign\&quot;,\n \&quot;campaignName\&quot;: \&quot;test campaign\&quot;,\n \&quot;wapUrl\&quot;:\&quot;www.mywebsite.com\&quot;,\n \&quot;recipients\&quot;: [\n \&quot;971554573936\&quot;\n ],\n \&quot;recipientFileUrl\&quot;: \n\&quot;\&quot;,\n \&quot;msg\&quot;: {\n \&quot;en\&quot;:\&quot;EngageX Health Check: ECM Bulk API Test\&quot;\n }\n}&quot;,
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
      <webElementGuid>987c3584-5c89-45d9-b2f1-5c94f9008d57</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${jwt_token}</value>
      <webElementGuid>91117eb2-128e-49c8-96fe-2157fc088987</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://smartmessaging.etisalat.ae:5676/campaigns/submissions/sms/b/1</restUrl>
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
      <id>5c73ffc4-57f5-44fa-8420-5acbabc4df67</id>
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
