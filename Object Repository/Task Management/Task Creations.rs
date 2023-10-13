<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Task Creations</name>
   <tag></tag>
   <elementGuidId>ab95a0f0-856b-453e-ae2a-889a6720ac04</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${TOKEN}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;management_work\&quot;: {\n        \&quot;task\&quot;: \&quot;${task}\&quot;,\n        \&quot;description\&quot;: \&quot;${description}\&quot;,\n        \&quot;start_at\&quot;: \&quot;${start_at}\&quot;,\n        \&quot;end_at\&quot;: \&quot;${end_at}\&quot;,\n        \&quot;user_id\&quot;: \&quot;${user_id}\&quot;\n    }\n}&quot;,
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
      <webElementGuid>a7076c1c-307f-48b6-9575-c40cb1684f2f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${TOKEN}</value>
      <webElementGuid>fef4e388-bdbd-45dd-8687-b1f5a22cc002</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BASE_URL}management_works</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>77d6195b-dd6a-4cd6-b367-3c6aaa73de9b</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authToken</defaultValue>
      <description></description>
      <id>9e04e34e-9d8d-42be-88bd-a45c90c5cd5f</id>
      <masked>false</masked>
      <name>TOKEN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
